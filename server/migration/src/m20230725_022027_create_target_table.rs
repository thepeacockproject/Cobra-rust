use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Target::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Target::Id)
                            .unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Target::Name).string_len(100).not_null())
                    .col(ColumnDef::new(Target::Weapon).integer().not_null())
                    .col(ColumnDef::new(Target::Outfit).integer().not_null())
                    .col(ColumnDef::new(Target::Ammo).tiny_unsigned().not_null())
                    .col(ColumnDef::new(Target::Situation).tiny_unsigned().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Target::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Target {
    Table,
    Id,
    Name,
    Weapon,
    Outfit,
    Ammo,
    Situation,
}
