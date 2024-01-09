use crate::database::utils::{create_index, create_table_if_not_exists, index_exists};
use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "search_history")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub search_query: String,
    pub last_search_time: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub(crate) async fn init(db: &DatabaseConnection) {
    create_table_if_not_exists(db, Entity).await;
    if !index_exists(db, "search_history", "idx_last_search_time").await {
        create_index(
            db,
            "search_history",
            vec!["last_search_time"],
            "idx_last_search_time",
        )
        .await;
    }
}
