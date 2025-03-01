use sea_orm::DatabaseConnection;

pub struct BingoTable<'db> {
  db: &'db DatabaseConnection,
}

impl<'db> BingoTable<'db> {
  pub fn new(db: &'db DatabaseConnection) -> Self {
    Self { db }
  }
}
