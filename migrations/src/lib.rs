pub use sea_orm_migration::prelude::*;

mod m20221113_000001_create_users_table;
mod m20221211_000002_create_id_version_index;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221113_000001_create_users_table::Migration),
            Box::new(m20221211_000002_create_id_version_index::Migration),
        ]
    }
}
