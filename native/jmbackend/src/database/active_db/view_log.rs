use crate::database::utils::{create_index, create_table_if_not_exists, index_exists};
use sea_orm::entity::prelude::*;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "view_log")]
pub struct Model {
    // 原漫画id
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub author: String,
    pub description: String,
    pub name: String,
    // 最后阅读或查看详情的时间
    pub last_view_time: i64,
    // 0 为没阅读过漫画
    pub last_view_chapter_id: i64,
    // 最后阅读过了第几页
    pub last_view_page: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub(crate) async fn init(db: &DatabaseConnection) {
    create_table_if_not_exists(db, Entity).await;
    if !index_exists(db, "view_log", "idx_last_view_time").await {
        create_index(db, "view_log", vec!["last_view_time"], "idx_last_view_time").await;
    }
}
