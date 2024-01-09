use sea_orm::entity::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::ConnectionTrait;
use sea_orm::DeleteResult;
use sea_orm::EntityTrait;
use sea_orm::QueryOrder;
use serde::{Deserialize, Serialize};

use crate::database::utils::{
    create_index, create_index_a, create_table_if_not_exists, index_exists,
};

use super::dl_chapter;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "dl_image")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub album_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub chapter_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub image_index: i64,
    // index
    pub name: String,
    // 说明
    // 如果album只有一个chapter的话 pub series: Vec<Series> 为空
    // 如果有多个章节的话, 第一个chapter的id与album一样
    // "JM_PAGE_IMAGE:{}:{}" , chapter_id, name
    pub key: String,
    /// (下载状态)
    /// 0:未下载, 1:下载成功 2:下载失败
    pub dl_status: i32,
    /// size
    pub width: u32,
    pub height: u32,
}

pub(crate) async fn init(db: &DatabaseConnection) {
    create_table_if_not_exists(db, Entity).await;
    if !index_exists(db, Entity {}.table_name(), "uk_chapter_id_image_index").await {
        // CREATE UNIQUE INDEX uk_chapter_id_image_index ON dl_image(chapter_id,image_index);
        create_index_a(
            db,
            Entity {}.table_name(),
            vec!["chapter_id", "image_index"],
            "uk_chapter_id_image_index",
            true,
        )
        .await;
    }
    if !index_exists(db, Entity {}.table_name(), "idx_name").await {
        create_index(db, Entity {}.table_name(), vec!["name"], "idx_name").await;
    }
    if !index_exists(db, Entity {}.table_name(), "idx_key").await {
        create_index(db, Entity {}.table_name(), vec!["key"], "idx_key").await;
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub(crate) async fn load_all_need_download_image(
    db: &impl ConnectionTrait,
    chapter: &dl_chapter::Model,
) -> Vec<Model> {
    Entity::find()
        .filter(Column::ChapterId.eq(chapter.id))
        .filter(Column::DlStatus.eq(0))
        .all(db)
        .await
        .unwrap()
}

pub(crate) async fn set_dl_status(
    db: &impl ConnectionTrait,
    chapter_id: i64,
    image_index: i64,
    status: i32,
    width: i32,
    height: i32,
) {
    Entity::update_many()
        .col_expr(Column::DlStatus, Expr::value(status))
        .col_expr(Column::Width, Expr::value(width))
        .col_expr(Column::Height, Expr::value(height))
        .filter(Column::ChapterId.eq(chapter_id))
        .filter(Column::ImageIndex.eq(image_index))
        .exec(db)
        .await
        .unwrap();
}

pub(crate) async fn renew_failed(db: &impl ConnectionTrait) {
    Entity::update_many()
        .col_expr(Column::DlStatus, Expr::value(0))
        .filter(Column::DlStatus.eq(2))
        .exec(db)
        .await
        .unwrap();
}

pub(crate) async fn has_not_success_images(db: &impl ConnectionTrait, chapter_id: i64) -> bool {
    Entity::find()
        .filter(Column::ChapterId.eq(chapter_id))
        .filter(Column::DlStatus.ne(1))
        .count(db)
        .await
        .unwrap()
        > 0
}

pub(crate) async fn find_by_key(db: &impl ConnectionTrait, key: &str) -> Option<Model> {
    Entity::find()
        .filter(Column::Key.eq(key))
        .one(db)
        .await
        .unwrap()
}

pub(crate) async fn find_by_chapter_id(db: &impl ConnectionTrait, chapter_id: i64) -> Vec<Model> {
    Entity::find()
        .filter(Column::ChapterId.eq(chapter_id))
        .order_by_asc(Column::ImageIndex)
        .all(db)
        .await
        .unwrap()
}

pub(crate) async fn delete_by_album_id(
    db: &impl ConnectionTrait,
    album_id: i64,
) -> Result<DeleteResult, DbErr> {
    Entity::delete_many()
        .filter(Column::AlbumId.eq(album_id))
        .exec(db)
        .await
}

pub(crate) async fn lisst_by_album_id(
    db: &impl ConnectionTrait,
    album_id: i64,
) -> Result<Vec<Model>, DbErr> {
    Ok(Entity::find()
        .filter(Column::AlbumId.eq(album_id))
        .all(db)
        .await?)
}
