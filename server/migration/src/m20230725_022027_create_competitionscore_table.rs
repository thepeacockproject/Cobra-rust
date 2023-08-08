use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(CompetitionScore::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CompetitionScore::Id)
                            .unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(CompetitionScore::SteamId)
                            .big_unsigned()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(CompetitionScore::Score)
                            .integer()
                            .not_null()
                        )
                    .col(
                        ColumnDef::new(CompetitionScore::Rating)
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
            .drop_table(Table::drop().table(CompetitionScore::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum CompetitionScore {
    Table,
    Id,
    SteamId,
    Score,
    Rating
}
