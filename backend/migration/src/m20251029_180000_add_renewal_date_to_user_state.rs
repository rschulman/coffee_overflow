use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(UserState::Table)
                    .add_column(date_null(UserState::RenewalDate))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(UserState::Table)
                    .drop_column(UserState::RenewalDate)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum UserState {
    Table,
    RenewalDate,
}
