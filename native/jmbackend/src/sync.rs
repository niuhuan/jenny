use crate::active_db::{view_log, view_log_tag, ACTIVE_DB};
use crate::Result;
use crate::SyncWebdav;
use futures_util::TryStreamExt;
use grouping_by::GroupingBy;
use itertools::Itertools;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::TransactionTrait;
use sea_orm::{ActiveModelTrait, IntoActiveModel};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::fmt::Write;
use std::ops::Deref;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_util::io::StreamReader;

pub(crate) async fn sync_webdav(json: &str) -> Result<String> {
    let config: SyncWebdav = from_str(json)?;
    let client = reqwest::Client::new();
    // 向下同步
    let mut req = client.request(reqwest::Method::GET, config.url.clone());
    if !config.username.is_empty() && !config.password.is_empty() {
        req = req.basic_auth(config.username.clone(), Some(config.password.clone()));
    }
    let rsp = req.send().await?;
    match rsp.status().as_u16() {
        200 => {
            let mut line = String::new();
            let mut lines = BufReader::new(StreamReader::new(
                rsp.bytes_stream()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)),
            ));
            loop {
                if lines.read_line(&mut line).await? == 0 {
                    break;
                }
                let str = line.trim();
                if str.is_empty() {
                    continue;
                }
                let vl_info: ViewLogInfo = from_str(str)?;
                upgrade(vl_info).await?;
                line.clear();
            }
        }
        404 => {}
        _ => {}
    }
    // 向上同步
    let last = load_last_1000_info().await?;
    let mut buf = bytes::BytesMut::new();
    for x in &last {
        buf.write_str(&to_string(x)?)?;
        buf.write_str("\n")?;
    }
    let mut req = client.request(reqwest::Method::PUT, config.url);
    if !config.username.is_empty() && !config.password.is_empty() {
        req = req.basic_auth(config.username, Some(config.password));
    }
    let rsp = req.body(buf.to_vec()).send().await?;
    rsp.error_for_status()?;
    Ok("".to_owned())
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
struct ViewLogInfo {
    pub view_log: view_log::Model,
    pub view_log_tags: Vec<view_log_tag::Model>,
}

async fn load_last_1000_info() -> Result<Vec<ViewLogInfo>> {
    let db = ACTIVE_DB.get().unwrap().lock().await;
    let logs: Vec<view_log::Model> = view_log::Entity::find()
        .order_by_desc(view_log::Column::LastViewTime)
        .all(db.deref())
        .await?;
    let log_ids = logs.iter().map(|e| e.id).collect_vec();
    let tags: Vec<view_log_tag::Model> = view_log_tag::Entity::find()
        .filter(view_log_tag::Column::Id.is_in(log_ids))
        .all(db.deref())
        .await?;
    let group = tags.iter().cloned().grouping_by(|t| t.id);
    Ok(logs
        .iter()
        .map(|log| ViewLogInfo {
            view_log: log.clone(),
            view_log_tags: if let Some(s) = group.get(&log.id) {
                s.clone()
            } else {
                vec![]
            },
        })
        .collect_vec())
}

async fn upgrade(vl_info: ViewLogInfo) -> Result<()> {
    let db = ACTIVE_DB.get().unwrap().lock().await;
    let in_db = view_log::Entity::find_by_id(vl_info.view_log.id.clone())
        .one(db.deref())
        .await?;
    if let Some(in_db) = in_db {
        if in_db.last_view_time < vl_info.view_log.last_view_time {
            db.transaction::<_, (), sea_orm::DbErr>(|txn| {
                Box::pin(async move {
                    let in_db_view_log = in_db;
                    let mut in_db_view_log: view_log::ActiveModel = in_db_view_log.into();
                    in_db_view_log.last_view_time = Set(vl_info.view_log.last_view_time);
                    in_db_view_log.last_view_chapter_id =
                        Set(vl_info.view_log.last_view_chapter_id);
                    in_db_view_log.last_view_page = Set(vl_info.view_log.last_view_page);
                    in_db_view_log.update(txn).await?;
                    Ok(())
                })
            })
            .await?;
        }
    } else {
        db.transaction::<_, (), sea_orm::DbErr>(|txn| {
            Box::pin(async move {
                // 插入主体
                let in_db_view_log = view_log::Model {
                    id: vl_info.view_log.id.clone(),
                    author: vl_info.view_log.author,
                    description: vl_info.view_log.description,
                    name: vl_info.view_log.name,
                    last_view_time: vl_info.view_log.last_view_time,
                    last_view_chapter_id: vl_info.view_log.last_view_chapter_id,
                    last_view_page: vl_info.view_log.last_view_page,
                };
                view_log::Entity::insert(in_db_view_log.into_active_model())
                    .exec(txn)
                    .await?;
                // 插入tag
                for tag in vl_info.view_log_tags {
                    view_log_tag::Entity::insert(tag.into_active_model())
                        .exec(txn)
                        .await?;
                }
                // ok
                Ok(())
            })
        })
        .await?;
    };
    Ok(())
}
