use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Message::Id)
                            .unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Message::FromId)
                            .big_unsigned()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Message::ToId)
                            .big_unsigned()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Message::IsRead)
                            .boolean()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Message::TemplateId)
                            .integer()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Message::TemplateData)
                            .text()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Message::Category)
                            .integer()
                            .not_null()
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Message {
    Table,
    Id,
    FromId,
    ToId,
    IsRead,
    TemplateId,
    TemplateData,
    Category

}
