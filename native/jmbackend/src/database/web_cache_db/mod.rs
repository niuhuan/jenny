use crate::database::utils::connect_db;
use crate::tools::join_paths;
use crate::{check_first, Result};
use once_cell::sync::OnceCell;
use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::Set;
use std::future::Future;
use std::ops::Deref;
use std::time::Duration;
use tokio::sync::Mutex;
pub(crate) mod web_cache;
use crate::take_hash_lock;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;

static WEB_CACHE_DB: OnceCell<Mutex<DatabaseConnection>> = OnceCell::new();

pub(crate) async fn init_db() {
    let path = join_paths(vec![crate::FOLDER.lock().await.deref(), "web_cache.db"]);
    let db = connect_db(&path).await;
    web_cache::init(&db).await;
    WEB_CACHE_DB
        .set(Mutex::new(db))
        .expect("INIT ACTIVE DB DUP");
}

pub(crate) async fn use_web_cache<Fut>(
    key: String,
    expire: Duration,
    reload: impl FnOnce() -> Fut,
) -> Result<String>
where
    Fut: Future<Output = Result<String>>,
{
    check_first().await?;
    // 时间
    let now = chrono::Local::now().timestamp();
    let earliest = now - (expire.as_secs() as i64);
    // 哈希锁
    let lock = take_hash_lock(key.clone()).await;
    // 读取数据库
    let db = WEB_CACHE_DB.get().unwrap().lock().await;
    let cache = web_cache::Entity::find()
        .filter(web_cache::Column::CacheKey.eq(key.clone()))
        // 如果框架支持upert的话
        // .filter(web_cache::Column::CacheTime.gt(&now.clone()))
        .one(db.deref())
        .await?;
    drop(db);
    if cache.is_some() {
        let cache = cache.clone().unwrap();
        if cache.cache_time > earliest {
            return Ok(cache.cache_content);
        }
    }
    let load = reload().await?;
    match cache {
        Some(cache) => {
            let mut data: web_cache::ActiveModel = cache.into();
            data.cache_content = Set(load.clone());
            data.cache_time = Set(now);
            let db = WEB_CACHE_DB.get().unwrap().lock().await;
            data.update(db.deref()).await?;
            drop(db);
        }
        None => {
            let data = web_cache::ActiveModel {
                cache_key: Set(key),
                cache_content: Set(load.clone()),
                cache_time: Set(now),
                ..Default::default()
            };
            let db = WEB_CACHE_DB.get().unwrap().lock().await;
            web_cache::Entity::insert(data).exec(db.deref()).await?;
            drop(db);
        }
    }
    drop(lock);
    Ok(load)
}

pub(crate) async fn clean_web_cache_by_patten(patten: String) -> Result<String> {
    let db = WEB_CACHE_DB.get().unwrap().lock().await;
    web_cache::Entity::delete_many()
        .filter(web_cache::Column::CacheKey.like(patten.as_str()))
        .exec(db.deref())
        .await?; // 不管有几条被作用
    drop(db);
    Ok("".to_owned())
}

pub(crate) async fn clean_all_web_cache() -> Result<String> {
    web_cache::Entity::delete_many()
        .exec(WEB_CACHE_DB.get().unwrap().lock().await.deref())
        .await?;
    Ok(String::default())
}

pub(crate) async fn clean_web_cache_by_time(time: i64) -> Result<String> {
    web_cache::Entity::delete_many()
        .filter(web_cache::Column::CacheTime.lt(time))
        .exec(WEB_CACHE_DB.get().unwrap().lock().await.deref())
        .await?;
    Ok(String::default())
}
