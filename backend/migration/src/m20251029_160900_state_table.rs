use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(State::Table)
                    .if_not_exists()
                    .col(pk_auto(State::Id))
                    .col(string(State::Name))
                    .col(integer(State::LegalHours))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserState::Table)
                    .if_not_exists()
                    .col(integer(UserState::UserId))
                    .col(integer(UserState::StateId))
                    .col(integer(UserState::HoursComplete))
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(UserState::Table)
                            .from_col(UserState::UserId)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .from_tbl(UserState::Table)
                            .from_col(UserState::StateId)
                            .to_tbl(State::Table)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(State::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserState::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum State {
    Table,
    Id,
    Name,
    LegalHours,
}

#[derive(DeriveIden)]
enum UserState {
    Table,
    UserId,
    StateId,
    HoursComplete,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}
