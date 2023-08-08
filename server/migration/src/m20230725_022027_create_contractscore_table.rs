use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(ContractScore::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ContractScore::Id)
                            .unsigned()
                            .not_null()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(ContractScore::SteamId)
                            .big_unsigned()
                            .not_null()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(ContractScore::Score)
                            .integer()
                            .not_null()
                        )
                    .col(
                        ColumnDef::new(ContractScore::Rating)
                            .integer()
                            .not_null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(ContractScore::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum ContractScore {
    Table,
    Id,
    SteamId,
    Score,
    Rating
}
