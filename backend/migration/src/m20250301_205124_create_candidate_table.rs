use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250301_204945_create_season_table::Season;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Candidate::Table)
          .if_not_exists()
          .col(pk_uuid(Candidate::Id))
          .col(string(Candidate::Name))
          .col(string(Candidate::Image))
          .col(uuid(Candidate::Season))
          .col(date_time(Candidate::Out))
          .foreign_key(
            ForeignKey::create()
              .from(Candidate::Table, Candidate::Season)
              .to(Season::Table, Season::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Candidate::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
pub enum Candidate {
  Table,
  Id,
  Name,
  Image,
  Season,
  Out,
}
