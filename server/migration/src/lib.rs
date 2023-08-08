pub use sea_orm_migration::prelude::*;

mod m20230725_022027_create_competition_table;
mod m20230725_022027_create_competitionscore_table;
mod m20230725_022027_create_contract_table;
mod m20230725_022027_create_contractscore_table;
mod m20230725_022027_create_message_table;
mod m20230725_022027_create_play_table;
mod m20230725_022027_create_queue_table;
mod m20230725_022027_create_rating_table;
mod m20230725_022027_create_sniperscore_table;
mod m20230725_022027_create_storyscore_table;
mod m20230725_022027_create_target_table;
mod m20230725_022027_create_tutorialscore_table;
mod m20230725_022027_create_user_table;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230725_022027_create_competition_table::Migration),
            Box::new(m20230725_022027_create_competitionscore_table::Migration),
            Box::new(m20230725_022027_create_contract_table::Migration),
            Box::new(m20230725_022027_create_contractscore_table::Migration),
            Box::new(m20230725_022027_create_message_table::Migration),
            Box::new(m20230725_022027_create_play_table::Migration),
            Box::new(m20230725_022027_create_queue_table::Migration),
            Box::new(m20230725_022027_create_rating_table::Migration),
            Box::new(m20230725_022027_create_sniperscore_table::Migration),
            Box::new(m20230725_022027_create_storyscore_table::Migration),
            Box::new(m20230725_022027_create_target_table::Migration),
            Box::new(m20230725_022027_create_tutorialscore_table::Migration),
            Box::new(m20230725_022027_create_user_table::Migration),
        ]
    }
}
