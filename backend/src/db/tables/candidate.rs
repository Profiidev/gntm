use sea_orm::DatabaseConnection;

pub struct CandidateTable<'db> {
  db: &'db DatabaseConnection,
}

impl<'db> CandidateTable<'db> {
  pub fn new(db: &'db DatabaseConnection) -> Self {
    Self { db }
  }
}
