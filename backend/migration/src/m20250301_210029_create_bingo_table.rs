use sea_orm_migration::{prelude::*, schema::*};

use crate::{
  m20250301_204945_create_season_table::Season, m20250301_205124_create_candidate_table::Candidate,
  m20250301_210020_create_user_table::User,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Bingo::Table)
          .if_not_exists()
          .col(pk_uuid(Bingo::Id))
          .col(uuid(Bingo::Season))
          .foreign_key(
            ForeignKey::create()
              .from(Bingo::Table, Bingo::Season)
              .to(Season::Table, Season::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(BingoCandidate::Table)
          .if_not_exists()
          .primary_key(
            Index::create()
              .table(BingoCandidate::Table)
              .col(BingoCandidate::Bingo)
              .col(BingoCandidate::Candidate),
          )
          .col(uuid(BingoCandidate::Bingo))
          .col(uuid(BingoCandidate::Candidate))
          .col(integer(BingoCandidate::Position))
          .foreign_key(
            ForeignKey::create()
              .from(BingoCandidate::Table, BingoCandidate::Bingo)
              .to(Bingo::Table, Bingo::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .from(BingoCandidate::Table, BingoCandidate::Candidate)
              .to(Candidate::Table, Candidate::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await?;

    manager
      .create_table(
        Table::create()
          .table(BingoUser::Table)
          .if_not_exists()
          .primary_key(
            Index::create()
              .table(BingoUser::Table)
              .col(BingoUser::Bingo)
              .col(BingoUser::User),
          )
          .col(uuid(BingoUser::Bingo))
          .col(uuid(BingoUser::User))
          .foreign_key(
            ForeignKey::create()
              .from(BingoUser::Table, BingoUser::Bingo)
              .to(Bingo::Table, Bingo::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .foreign_key(
            ForeignKey::create()
              .from(BingoUser::Table, BingoUser::User)
              .to(User::Table, User::Id)
              .on_delete(ForeignKeyAction::Cascade)
              .on_update(ForeignKeyAction::Cascade),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(
        Table::drop()
          .table(Bingo::Table)
          .table(BingoCandidate::Table)
          .table(BingoUser::Table)
          .to_owned(),
      )
      .await
  }
}

#[derive(DeriveIden)]
pub enum Bingo {
  Table,
  Id,
  Season,
}

#[derive(DeriveIden)]
enum BingoCandidate {
  Table,
  Bingo,
  Candidate,
  Position,
}

#[derive(DeriveIden)]
enum BingoUser {
  Table,
  Bingo,
  User,
}
