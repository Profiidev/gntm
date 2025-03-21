use bingo::BingoTable;
use candidate::CandidateTable;
use invalid_jwt::InvalidJwtTable;
use key::KeyTable;
use sea_orm::DatabaseConnection;
use season::SeasonTable;
use user::UserTable;

mod bingo;
mod candidate;
mod invalid_jwt;
mod key;
mod season;
mod user;

pub struct Tables<'db> {
  db: &'db DatabaseConnection,
}

impl<'db> Tables<'db> {
  pub fn new(db: &'db DatabaseConnection) -> Self {
    Self { db }
  }

  pub fn user(self) -> UserTable<'db> {
    UserTable::new(self.db)
  }

  pub fn season(self) -> SeasonTable<'db> {
    SeasonTable::new(self.db)
  }

  pub fn candidate(self) -> CandidateTable<'db> {
    CandidateTable::new(self.db)
  }

  pub fn bingo(self) -> BingoTable<'db> {
    BingoTable::new(self.db)
  }

  pub fn key(self) -> KeyTable<'db> {
    KeyTable::new(self.db)
  }

  pub fn invalid_jwt(self) -> InvalidJwtTable<'db> {
    InvalidJwtTable::new(self.db)
  }
}
