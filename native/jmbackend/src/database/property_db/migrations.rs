use sea_orm::{ConnectionTrait, Statement};
use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(M20220305V100CleanCookie)]
    }
}

pub struct M20220305V100CleanCookie;

impl MigrationName for M20220305V100CleanCookie {
    fn name(&self) -> &str {
        "M20220305V100CleanCookie"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for M20220305V100CleanCookie {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DELETE FROM property WHERE k = 'cookie'";
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
