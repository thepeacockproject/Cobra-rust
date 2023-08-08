//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "queue")]
pub struct Model {
    #[sea_orm(column_name = "steamId", primary_key, auto_increment = false)]
    pub steam_id: u64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: u32,
    #[sea_orm(column_name = "isCompetition")]
    pub is_competition: bool,
    #[sea_orm(column_name = "createdAt")]
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::SteamId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
