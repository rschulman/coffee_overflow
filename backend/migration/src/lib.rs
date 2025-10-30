pub use sea_orm_migration::prelude::*;

mod m20251028_205246_add_users_table;
mod m20251029_160900_state_table;
mod m20251029_170000_add_session_table;
mod m20251029_180000_add_renewal_date_to_user_state;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251028_205246_add_users_table::Migration),
            Box::new(m20251029_160900_state_table::Migration),
            Box::new(m20251029_170000_add_session_table::Migration),
            Box::new(m20251029_180000_add_renewal_date_to_user_state::Migration),
        ]
    }
}
