use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .big_unsigned()
                            .not_null()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(User::Wallet)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(User::ContractPlays)
                            .integer()
                            .not_null()
                        )
                    .col(
                        ColumnDef::new(User::Trophies)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(User::CompetitionPlays)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(User::Country)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(User::DisplayName)
                            .string_len(32)
                            .not_null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    Table,
    Id,
    Wallet,
    ContractPlays,
    Trophies,
    CompetitionPlays,
    Country,
    DisplayName
}
