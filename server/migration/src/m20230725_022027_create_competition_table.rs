use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Competition::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Competition::Id)
                            .unsigned()
                            .not_null()
                            .primary_key()
                    )
                    .col(
                        ColumnDef::new(Competition::ContractId)
                            .unsigned()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Competition::EndTime)
                            .timestamp()
                            .not_null()
                        )
                    .col(
                        ColumnDef::new(Competition::AllowInvites)
                            .boolean()
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Competition::Creator)
                            .big_unsigned()
                            .not_null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Competition::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Competition {
    Table,
    Id,
    ContractId,
    EndTime,
    AllowInvites,
    Creator
    
}
