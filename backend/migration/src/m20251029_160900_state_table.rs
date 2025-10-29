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

        // Insert all 50 US states with their two-letter codes
        let insert_states = Query::insert()
            .into_table(State::Table)
            .columns([State::Name, State::LegalHours])
            .values_panic(["AL".into(), 12.into()])
            .values_panic(["AK".into(), 12.into()])
            .values_panic(["AZ".into(), 15.into()])
            .values_panic(["AR".into(), 12.into()])
            .values_panic(["CA".into(), 9.into()])
            .values_panic(["CO".into(), 45.into()])
            .values_panic(["CT".into(), 12.into()])
            .values_panic(["DE".into(), 24.into()])
            .values_panic(["FL".into(), 33.into()])
            .values_panic(["GA".into(), 12.into()])
            .values_panic(["HI".into(), 0.into()])
            .values_panic(["ID".into(), 30.into()])
            .values_panic(["IL".into(), 30.into()])
            .values_panic(["IN".into(), 36.into()])
            .values_panic(["IA".into(), 15.into()])
            .values_panic(["KS".into(), 12.into()])
            .values_panic(["KY".into(), 13.into()])
            .values_panic(["LA".into(), 15.into()])
            .values_panic(["ME".into(), 11.into()])
            .values_panic(["MD".into(), 0.into()])
            .values_panic(["MA".into(), 0.into()])
            .values_panic(["MI".into(), 0.into()])
            .values_panic(["MN".into(), 45.into()])
            .values_panic(["MS".into(), 12.into()])
            .values_panic(["MO".into(), 15.into()])
            .values_panic(["MT".into(), 15.into()])
            .values_panic(["NE".into(), 10.into()])
            .values_panic(["NV".into(), 13.into()])
            .values_panic(["NH".into(), 12.into()])
            .values_panic(["NJ".into(), 24.into()])
            .values_panic(["NM".into(), 15.into()])
            .values_panic(["NY".into(), 24.into()])
            .values_panic(["NC".into(), 12.into()])
            .values_panic(["ND".into(), 45.into()])
            .values_panic(["OH".into(), 24.into()])
            .values_panic(["OK".into(), 12.into()])
            .values_panic(["OR".into(), 45.into()])
            .values_panic(["PA".into(), 12.into()])
            .values_panic(["RI".into(), 10.into()])
            .values_panic(["SC".into(), 14.into()])
            .values_panic(["SD".into(), 0.into()])
            .values_panic(["TN".into(), 15.into()])
            .values_panic(["TX".into(), 15.into()])
            .values_panic(["UT".into(), 24.into()])
            .values_panic(["VT".into(), 20.into()])
            .values_panic(["VA".into(), 12.into()])
            .values_panic(["WA".into(), 45.into()])
            .values_panic(["WV".into(), 24.into()])
            .values_panic(["WI".into(), 30.into()])
            .values_panic(["WY".into(), 15.into()])
            .to_owned();

        manager.exec_stmt(insert_states).await?;

        manager
            .create_table(
                Table::create()
                    .table(UserState::Table)
                    .if_not_exists()
                    .col(pk_auto(UserState::Id))
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
    Id,
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
