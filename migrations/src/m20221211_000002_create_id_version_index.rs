use entities::user;
use sea_orm_migration::prelude::*;
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20221211_create_version_index"
    }
}

const IDX_NAME: &str = "user_id_version_idx";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name(IDX_NAME)
                    .unique()
                    .table(user::Entity)
                    .col(user::Column::Id)
                    .col(user::Column::Version)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name(IDX_NAME).table(user::Entity).to_owned())
            .await
    }
}
