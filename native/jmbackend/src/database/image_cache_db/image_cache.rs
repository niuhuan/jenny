use crate::database::utils::{create_index, create_table_if_not_exists, index_exists};
use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "image_cache")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub cache_key: String,
    pub cache_path: String,
    pub cache_time: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

//  CREATE INDEX idx_cache_time ON image_cache(cache_time);
//  select * from sqlite_master where type='index' AND tbl_name='image_cache' AND name='idx_cache_time';

pub(crate) async fn init(db: &DatabaseConnection) {
    create_table_if_not_exists(db, Entity).await;
    if !index_exists(db, "image_cache", "idx_cache_time").await {
        create_index(db, "image_cache", vec!["cache_time"], "idx_cache_time").await;
    }
}
