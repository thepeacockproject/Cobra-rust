use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TutorialScore::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TutorialScore::Id)
                            .char_len(7)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TutorialScore::SteamId)
                            .big_unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TutorialScore::Score).integer().not_null())
                    .col(ColumnDef::new(TutorialScore::Rating).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(TutorialScore::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum TutorialScore {
    Table,
    Id,
    SteamId,
    Score,
    Rating,
}
