use super::dl_album;
use crate::database::utils::{create_index, create_table_if_not_exists, index_exists};
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::ConnectionTrait;
use sea_orm::DeleteResult;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Eq, Hash, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "dl_chapter")]
pub struct Model {
    pub album_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub name: String,
    pub sort: String,
    /// 0:未加载图片 1:已加载图片
    pub load_images: i32,
    /// 图片总数
    pub image_count: i32,
    /// 下载了的图片总数
    pub dled_image_count: i32,
    /// image(图片的下载状态)
    /// 0:未下载, 1:全部下载成功 2:任何一个下载失败
    // "JM_PAGE_IMAGE:{}:{}"
    pub dl_status: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub(crate) async fn init(db: &DatabaseConnection) {
    create_table_if_not_exists(db, Entity).await;
    if !index_exists(db, Entity {}.table_name(), "idx_album_id").await {
        create_index(db, Entity {}.table_name(), vec!["album_id"], "idx_album_id").await;
    }
}

pub(crate) async fn load_all_need_download_chapter(
    db: &impl ConnectionTrait,
    album: &dl_album::Model,
) -> Vec<Model> {
    Entity::find()
        .filter(Column::AlbumId.eq(album.id))
        .filter(Column::DlStatus.eq(0))
        .all(db)
        .await
        .unwrap()
}

pub(crate) async fn set_dl_status(db: &impl ConnectionTrait, id: i64, status: i32) {
    Entity::update_many()
        .col_expr(Column::DlStatus, Expr::value(status))
        .filter(Column::Id.eq(id))
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

pub(crate) async fn save_image_count(db: &impl ConnectionTrait, id: i64, count: i32) {
    Entity::update_many()
        .col_expr(Column::LoadImages, Expr::value(1))
        .col_expr(Column::ImageCount, Expr::value(count))
        .filter(Column::Id.eq(id))
        .exec(db)
        .await
        .unwrap();
}

pub(crate) async fn has_not_success_chapter(db: &impl ConnectionTrait, album_id: i64) -> bool {
    Entity::find()
        .filter(Column::AlbumId.eq(album_id))
        .filter(Column::DlStatus.ne(1))
        .count(db)
        .await
        .unwrap()
        > 0
}

pub(crate) async fn find_by_id(db: &impl ConnectionTrait, id: i64) -> Option<Model> {
    Entity::find_by_id(id).one(db).await.unwrap()
}

pub(crate) async fn list_by_album_id(db: &impl ConnectionTrait, album_id: i64) -> Vec<Model> {
    Entity::find()
        .filter(Column::AlbumId.eq(album_id))
        .all(db)
        .await
        .unwrap()
}

pub(crate) async fn download_one_image(db: &impl ConnectionTrait, id: i64) {
    Entity::update_many()
        .col_expr(
            Column::DledImageCount,
            Expr::col(Column::DledImageCount).add(1),
        )
        .filter(Column::Id.eq(id))
        .exec(db)
        .await
        .unwrap();
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
