use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SniperScore::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SniperScore::SteamId)
                            .unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SniperScore::Score).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SniperScore::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum SniperScore {
    Table,
    SteamId,
    Score,
}
