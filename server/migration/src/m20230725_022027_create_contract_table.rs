use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Contract::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Contract::Id)
                            .unsigned()
                            .not_null()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(Contract::DisplayId)
                            .string_len(18)
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Contract::Title)
                            .string_len(30)
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Contract::Description)
                            .string_len(350)
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Contract::SteamId)
                            .big_unsigned()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Contract::LevelIdx)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Contract::CheckpointIdx)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Contract::Difficulty)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Contract::ExitId)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Contract::WeaponToken)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Contract::OutfitToken)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Contract::Target1)
                            .unsigned()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Contract::Target2)
                            .unsigned()
                    )
                    .col(
                        ColumnDef::new(Contract::Target3)
                            .unsigned()
                    )
                    .col(
                        ColumnDef::new(Contract::Restrictions)
                            .tiny_unsigned()
                            .not_null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Contract::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Contract {
    Table,
    Id,
    DisplayId,
    Title,
    Description,
    SteamId,
    LevelIdx,
    CheckpointIdx,
    Difficulty,
    ExitId,
    WeaponToken,
    OutfitToken,
    Target1,
    Target2,
    Target3,
    Restrictions
}
