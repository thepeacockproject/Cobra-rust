use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Queue::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Queue::SteamId)
                            .big_unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Queue::Id)
                            .unsigned()
                            .not_null()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(Queue::IsCompetition)
                            .boolean()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Queue::CreatedAt)
                            .timestamp()
                            .not_null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Queue::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Queue {
    Table,
    SteamId,
    Id,
    IsCompetition,
    CreatedAt,
}
