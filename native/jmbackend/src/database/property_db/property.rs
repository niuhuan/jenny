use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;

use crate::database::utils::{create_index, create_table_if_not_exists, index_exists};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "property")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub k: String,
    pub v: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub(crate) async fn init(db: &DatabaseConnection) {
    create_table_if_not_exists(&db, Entity).await;
    if !index_exists(db, "property", "idx_k").await {
        create_index(db, "property", vec!["k"], "idx_k").await;
    }
}
