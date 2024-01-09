use std::ops::Deref;

use once_cell::sync::OnceCell;
use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::Set;
use sea_orm_migration::MigratorTrait;
use tokio::sync::Mutex;

use crate::database::utils::connect_db;
use crate::tools::join_paths;
use crate::Result;

pub(crate) mod property;

pub(crate) mod migrations;

static PROPERTY_DB: OnceCell<Mutex<DatabaseConnection>> = OnceCell::new();

pub(crate) async fn init_db() {
    let path = join_paths(vec![crate::FOLDER.lock().await.deref(), "property.db"]);
    let db = connect_db(&path).await;
    property::init(&db).await;
    migrations::Migrator::up(&db, None).await.unwrap();
    PROPERTY_DB.set(Mutex::new(db)).expect("INIT ACTIVE DB DUP");
}

pub(crate) async fn save_property(k: String, v: String) -> Result<String> {
    let db = PROPERTY_DB.get().unwrap().lock().await;
    let in_db = property::Entity::find_by_id(k.clone())
        .one(db.deref())
        .await?;
    match in_db {
        Some(in_db) => {
            let mut data: property::ActiveModel = in_db.into();
            data.k = Set(k.clone());
            data.v = Set(v.clone());
            data.update(db.deref()).await?;
        }
        None => {
            let insert = property::ActiveModel {
                k: Set(k.clone()),
                v: Set(v.clone()),
                ..Default::default()
            };
            insert.insert(db.deref()).await?;
        }
    };
    drop(db);
    Ok("".to_string())
}

pub(crate) async fn load_property(k: String) -> Result<String> {
    let db = PROPERTY_DB.get().unwrap().lock().await;
    let in_db = property::Entity::find_by_id(k.clone())
        .one(db.deref())
        .await?;
    let v = match in_db {
        Some(in_db) => in_db.v,
        None => String::default(),
    };
    drop(db);
    Ok(v)
}

pub(crate) async fn load_int_property(k: String, default: i64) -> i64 {
    match load_property(k).await {
        Ok(p) => match p.parse::<i64>() {
            Ok(data) => data,
            Err(_) => default,
        },
        Err(_) => default,
    }
}
