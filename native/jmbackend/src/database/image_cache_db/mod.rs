use crate::database::utils::connect_db;
use crate::tools::join_paths;
use crate::Result;
use once_cell::sync::OnceCell;
use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::Set;
use std::future::Future;
use std::ops::Deref;
use tokio::sync::Mutex;
pub(crate) mod image_cache;
use crate::take_hash_lock;
use bytes::Bytes;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::QuerySelect;
use std::pin::Pin;

static IMAGE_CACHE_DB: OnceCell<Mutex<DatabaseConnection>> = OnceCell::new();

pub(crate) async fn init_db() {
    let path = join_paths(vec![crate::FOLDER.lock().await.deref(), "image_cache.db"]);
    let db = connect_db(&path).await;
    image_cache::init(&db).await;
    IMAGE_CACHE_DB
        .set(Mutex::new(db))
        .expect("INIT ACTIVE DB DUP");
}

static IMAGE_CACHE_FOLDER: OnceCell<String> = OnceCell::new();

pub(crate) async fn init_dir() {
    let dir = join_paths(vec![crate::FOLDER.lock().await.deref(), "image_cache"]);
    tokio::fs::create_dir_all(dir.clone()).await.unwrap();
    IMAGE_CACHE_FOLDER.set(dir).expect("INIT ACTIVE DB DUP");
}

pub(crate) async fn use_image_cache<F>(key: String, f: Pin<Box<F>>) -> Result<String>
where
    F: Future<Output = Result<(Bytes, u32, u32)>>,
{
    // 哈希锁
    let lock = take_hash_lock(key.clone()).await;
    // 查找图片是否有缓存
    let db = IMAGE_CACHE_DB.get().unwrap().lock().await;
    let db_image: Option<image_cache::Model> = image_cache::Entity::find_by_id(key.clone())
        .one(db.deref())
        .await?;
    drop(db);
    let path = match db_image {
        // 有缓存直接使用
        Some(db_image) => db_image.cache_path,
        // 没有缓存则下载
        None => {
            let data: (Bytes, u32, u32) = f.await?;
            let now = chrono::Local::now().timestamp();
            let path = format!(
                "{}{}",
                hex::encode(md5::compute(key.clone()).to_vec()),
                &now,
            );
            let cache_folder = IMAGE_CACHE_FOLDER.get().unwrap();
            let local = join_paths(vec![&cache_folder, &path.clone()]);
            // drop(cache_folder);
            std::fs::write(local, data.0)?;
            let insert = image_cache::ActiveModel {
                cache_key: Set(key.clone()),
                cache_path: Set(path.clone()),
                cache_time: Set(now.clone()),
                ..Default::default()
            };
            let db = IMAGE_CACHE_DB.get().unwrap().lock().await;
            insert.insert(db.deref()).await?;
            drop(db);
            path
        }
    };
    let cache_folder = IMAGE_CACHE_FOLDER.get().unwrap();
    let local = join_paths(vec![&cache_folder, &path]);
    // drop(cache_folder);
    drop(lock);
    Ok(local)
}

pub(crate) async fn clean_all_image_cache() -> Result<String> {
    clean_image_cache_by_time(chrono::Local::now().timestamp()).await
}

pub(crate) async fn clean_image_cache_by_time(time: i64) -> Result<String> {
    let cache_folder = IMAGE_CACHE_FOLDER.get().unwrap();
    let dir = cache_folder.clone();
    // drop(cache_folder);
    let db = IMAGE_CACHE_DB.get().unwrap().lock().await;
    loop {
        let caches: Vec<image_cache::Model> = image_cache::Entity::find()
            .filter(image_cache::Column::CacheTime.lt(time))
            .limit(100)
            .all(db.deref())
            .await?;
        if caches.is_empty() {
            break;
        }
        for cache in caches {
            let local = join_paths(vec![
                dir.clone().as_str(),
                cache.cache_path.clone().as_str(),
            ]);
            image_cache::Entity::delete_many()
                .filter(image_cache::Column::CacheKey.eq(cache.cache_key))
                .exec(db.deref())
                .await?; // 不管有几条被作用
            let _ = std::fs::remove_file(local); // 不管成功与否
        }
    }
    Ok(String::default())
}
