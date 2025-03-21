pub use sea_orm_migration::prelude::*;

mod m20250301_204945_create_season_table;
mod m20250301_205124_create_candidate_table;
mod m20250301_210020_create_user_table;
mod m20250301_210029_create_bingo_table;
mod m20250301_215149_create_key_table;
mod m20250301_215333_create_invalid_jwt_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
      Box::new(m20250301_204945_create_season_table::Migration),
      Box::new(m20250301_205124_create_candidate_table::Migration),
      Box::new(m20250301_210020_create_user_table::Migration),
      Box::new(m20250301_210029_create_bingo_table::Migration),
      Box::new(m20250301_215149_create_key_table::Migration),
      Box::new(m20250301_215333_create_invalid_jwt_table::Migration),
    ]
  }
}
