use crate::database::utils::connect_db;
use crate::tools::join_paths;
use crate::{is_pro, Result, NO_PRO_MAX};
use anyhow::anyhow;
use jmcomic::ComicSimple;
use jmcomic::SearchPage;
use once_cell::sync::OnceCell;
use sea_orm::sea_query::Expr;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::ConnectionTrait;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use sea_orm::EntityName;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;
use sea_orm::QuerySelect;
use sea_orm::Set;
use sea_orm::Statement;
use sea_orm::TransactionTrait;
use std::ops::Deref;
use tokio::sync::Mutex;

pub(crate) mod dl_album;
pub(crate) mod dl_chapter;
pub(crate) mod dl_image;
pub(crate) mod search_history;
pub(crate) mod view_log;
pub(crate) mod view_log_tag;

pub(crate) static ACTIVE_DB: OnceCell<Mutex<DatabaseConnection>> = OnceCell::new();

pub(crate) async fn init_db() {
    let path = join_paths(vec![crate::FOLDER.lock().await.deref(), "active.db"]);
    let db = connect_db(&path).await;
    view_log::init(&db).await;
    view_log_tag::init(&db).await;
    search_history::init(&db).await;
    dl_album::init(&db).await;
    dl_chapter::init(&db).await;
    dl_image::init(&db).await;
    ACTIVE_DB.set(Mutex::new(db)).expect("INIT ACTIVE DB DUP");
}

pub(crate) async fn last_view_album(album: jmcomic::ComicAlbumResponse) -> Result<()> {
    let db = ACTIVE_DB.get().unwrap().lock().await;
    db.transaction::<_, (), sea_orm::DbErr>(|txn| {
        Box::pin(async move {
            let in_db_view_log: Option<view_log::Model> =
                view_log::Entity::find_by_id(album.id.clone())
                    .one(txn)
                    .await?;
            match in_db_view_log {
                Some(in_db_view_log) => {
                    let mut in_db_view_log: view_log::ActiveModel = in_db_view_log.into();
                    in_db_view_log.last_view_time = Set(chrono::Local::now().timestamp());
                    in_db_view_log.update(txn).await?;
                }
                None => {
                    view_log::ActiveModel {
                        id: Set(album.id),
                        author: Set(album.author.join(",")),
                        description: Set(album.description),
                        name: Set(album.name),
                        last_view_time: Set(chrono::Local::now().timestamp()),
                        last_view_chapter_id: Set(0),
                        last_view_page: Set(0),
                        ..Default::default()
                    }
                    .insert(txn)
                    .await?;
                    let mut tags: Vec<String> = vec![];
                    for tag_name in album.tags.clone() {
                        if !tags.contains(&tag_name) {
                            tags.push(tag_name);
                        }
                    }
                    for tag_name in tags {
                        view_log_tag::ActiveModel {
                            id: Set(album.id),
                            tag_name: Set(tag_name),
                            ..Default::default()
                        }
                        .insert(txn)
                        .await?;
                    }
                }
            };
            Ok(())
        })
    })
    .await?;
    Ok(())
}

pub(crate) async fn update_view_log(query: crate::types::UpdateViewLogQuery) -> Result<String> {
    view_log::Entity::update_many()
        .col_expr(
            view_log::Column::LastViewTime,
            Expr::value(chrono::Local::now().timestamp()),
        )
        .col_expr(
            view_log::Column::LastViewChapterId,
            Expr::value(query.last_view_chapter_id),
        )
        .col_expr(
            view_log::Column::LastViewPage,
            Expr::value(query.last_view_page),
        )
        .filter(view_log::Column::Id.eq(query.id))
        .exec(ACTIVE_DB.get().unwrap().lock().await.deref())
        .await?;
    Ok(String::default())
}

pub(crate) async fn find_view_log(id: i64) -> Result<Option<view_log::Model>> {
    Ok(view_log::Entity::find_by_id(id)
        .one(ACTIVE_DB.get().unwrap().lock().await.deref())
        .await?)
}

pub(crate) async fn page_view_log(page_number: i64) -> Result<SearchPage<ComicSimple>> {
    if !is_pro().await?.is_pro && page_number > NO_PRO_MAX {
        return Err(anyhow!("需要发电鸭"));
    }
    let active_db = ACTIVE_DB.get().unwrap().lock().await;
    let stmt = Statement::from_string(
        active_db.get_database_backend(),
        format!(
            "SELECT COUNT(*) AS c FROM {};",
            view_log::Entity {}.table_name(),
        ),
    );
    let rsp = active_db.query_one(stmt).await?.unwrap();
    let total: i32 = rsp.try_get("", "c")?;
    let page_size = 20;
    let list: Vec<view_log::Model> = view_log::Entity::find()
        .order_by_desc(view_log::Column::LastViewTime)
        .offset(Some(((page_number - 1) * page_size).try_into()?))
        .limit(Some(page_size.try_into()?))
        .all(active_db.deref())
        .await?;
    let vec = list
        .iter()
        .map(|model| ComicSimple {
            id: model.id,
            author: model.author.clone(),
            description: model.description.clone(),
            name: model.name.clone(),
            image: "".to_string(),
            category: Default::default(),
            category_sub: Default::default(),
        })
        .collect::<Vec<ComicSimple>>();
    Ok(SearchPage {
        search_query: "".to_string(),
        total: total.try_into()?,
        content: vec,
        redirect_aid: None,
    })
}

pub(crate) async fn clear_view_log() -> Result<String> {
    let db = ACTIVE_DB.get().unwrap().lock().await;
    db.transaction::<_, (), sea_orm::DbErr>(|txn| {
        Box::pin(async move {
            view_log::Entity::delete_many().exec(txn).await?;
            view_log_tag::Entity::delete_many().exec(txn).await?;
            Ok(())
        })
    })
    .await?;
    let stmt = Statement::from_string(db.get_database_backend(), "VACUUM;".to_owned());
    db.query_one(stmt).await?;
    Ok(String::default())
}

pub(crate) async fn db_clear_all_search_log() -> Result<()> {
    let db = ACTIVE_DB.get().unwrap().lock().await;
    search_history::Entity::delete_many()
        .exec(db.deref())
        .await?;
    Ok(())
}

pub(crate) async fn db_clear_a_search_log(content: String) -> Result<()> {
    let db = ACTIVE_DB.get().unwrap().lock().await;
    search_history::Entity::delete_many()
        .filter(search_history::Column::SearchQuery.eq(content))
        .exec(db.deref())
        .await?;
    Ok(())
}

pub(crate) async fn save_search_history(search_query: String) -> Result<()> {
    let db = ACTIVE_DB.get().unwrap().lock().await;
    let in_db = search_history::Entity::find_by_id(search_query.clone())
        .one(db.deref())
        .await?;
    match in_db {
        Some(in_db) => {
            let mut data: search_history::ActiveModel = in_db.into();
            data.last_search_time = Set(chrono::Local::now().timestamp());
            data.update(db.deref()).await?;
        }
        None => {
            let insert = search_history::ActiveModel {
                search_query: Set(search_query),
                last_search_time: Set(chrono::Local::now().timestamp()),
                ..Default::default()
            };
            insert.insert(db.deref()).await?;
        }
    };
    drop(db);
    Ok(())
}

pub(crate) async fn load_last_search_histories(limit: i64) -> Result<Vec<search_history::Model>> {
    let db = ACTIVE_DB.get().unwrap().lock().await;
    let list: Vec<search_history::Model> = search_history::Entity::find()
        .order_by_desc(search_history::Column::LastSearchTime)
        .offset(0)
        .limit(Some(limit.try_into()?))
        .all(db.deref())
        .await?;
    Ok(list)
}

pub(crate) async fn clear_download_album(id: i64) {
    let db = ACTIVE_DB.get().unwrap().lock().await;
    db.transaction::<_, (), DbErr>(|db| {
        Box::pin(async move {
            dl_image::delete_by_album_id(db, id.clone()).await?;
            dl_chapter::delete_by_album_id(db, id.clone()).await?;
            dl_album::delete_by_album_id(db, id.clone()).await?;
            Ok(())
        })
    })
    .await
    .unwrap();
}
