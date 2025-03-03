use sea_orm::DatabaseConnection;

pub struct SeasonTable<'db> {
  db: &'db DatabaseConnection,
}

impl<'db> SeasonTable<'db> {
  pub fn new(db: &'db DatabaseConnection) -> Self {
    Self { db }
  }
}
