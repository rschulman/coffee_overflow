use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(string(Session::Token).primary_key())
                    .col(integer(Session::UserId))
                    .col(timestamp(Session::CreatedAt))
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(Session::Table)
                            .from_col(Session::UserId)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Session {
    Table,
    Token,
    UserId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}