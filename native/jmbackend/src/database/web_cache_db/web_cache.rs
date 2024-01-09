use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;

use crate::database::utils::{create_index, create_table_if_not_exists, index_exists};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "web_cache")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub cache_key: String,
    pub cache_content: String,
    pub cache_time: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub(crate) async fn init(db: &DatabaseConnection) {
    create_table_if_not_exists(db, Entity).await;
    if !index_exists(db, "web_cache", "idx_cache_time").await {
        create_index(db, "web_cache", vec!["cache_time"], "idx_cache_time").await;
    }
}
