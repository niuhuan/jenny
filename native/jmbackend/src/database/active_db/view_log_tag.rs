use crate::database::utils::create_table_if_not_exists;
use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "view_log_tag")]
pub struct Model {
    // 原漫画id
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(primary_key)]
    pub tag_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub(crate) async fn init(db: &DatabaseConnection) {
    create_table_if_not_exists(db, Entity).await;
}
