use crate::database::utils::create_table_if_not_exists;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::ColumnTrait;
use sea_orm::ConnectionTrait;
use sea_orm::DatabaseTransaction;
use sea_orm::DeleteResult;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QuerySelect;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "dl_album")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub name: String,
    /// JSON(Vec<String>)
    pub author: String,
    /// JSON(Vec<String>)
    pub tags: String,
    /// JSON(Vec<String>)
    pub works: String,
    pub description: String,
    /// 方形状的封面下载状态
    /// 0:未下载, 1:下载成功 2:下载失败
    pub dl_square_cover_status: i32,
    /// 3x4的封面下载状态
    /// 0:未下载, 1:下载成功 2:下载失败
    pub dl_3x4_cover_status: i32,
    /// chapter(所有章节的下载状态)
    /// 0:未下载, 1:全部下载成功 2:任何一个下载失败 3:删除中
    pub dl_status: i32,
    /// 图片总数
    pub image_count: i32,
    /// 下载了的图片总数
    pub dled_image_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub(crate) async fn init(db: &DatabaseConnection) {
    create_table_if_not_exists(db, Entity).await;
}

pub(crate) async fn load_first_need_download_album(db: &DatabaseConnection) -> Option<Model> {
    Entity::find()
        .filter(Column::DlStatus.eq(0))
        .limit(1)
        .one(db)
        .await
        .unwrap()
}

pub(crate) async fn load_first_need_delete_album(db: &DatabaseConnection) -> Option<Model> {
    Entity::find()
        .filter(Column::DlStatus.eq(3))
        .limit(1)
        .one(db)
        .await
        .unwrap()
}

pub(crate) async fn inc_image_count(db: &DatabaseTransaction, id: i64, count: i32) {
    Entity::update_many()
        .col_expr(Column::ImageCount, Expr::col(Column::ImageCount).add(count))
        .filter(Column::Id.eq(id))
        .exec(db)
        .await
        .unwrap();
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

pub(crate) async fn set_3x4_cover_status(db: &impl ConnectionTrait, id: i64, status: i32) {
    Entity::update_many()
        .col_expr(Column::Dl3x4CoverStatus, Expr::value(status))
        .filter(Column::Id.eq(id))
        .exec(db)
        .await
        .unwrap();
}

pub(crate) async fn set_square_cover_status(db: &impl ConnectionTrait, id: i64, status: i32) {
    Entity::update_many()
        .col_expr(Column::DlSquareCoverStatus, Expr::value(status))
        .filter(Column::Id.eq(id))
        .exec(db)
        .await
        .unwrap();
}

pub(crate) async fn find_by_id(db: &impl ConnectionTrait, id: i64) -> Option<Model> {
    Entity::find_by_id(id).one(db).await.unwrap()
}

pub(crate) async fn all(db: &impl ConnectionTrait) -> Vec<Model> {
    Entity::find().all(db).await.unwrap()
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
        .filter(Column::Id.eq(album_id))
        .exec(db)
        .await
}
