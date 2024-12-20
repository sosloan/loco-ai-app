#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20231220_000001_agents;
mod m20231220_000002_tasks;
mod m20231220_000003_memory;
mod m20231220_000004_knowledge;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20231220_000001_agents::Migration),
            Box::new(m20231220_000002_tasks::Migration),
            Box::new(m20231220_000003_memory::Migration),
            Box::new(m20231220_000004_knowledge::Migration),
        ]
    }
}
