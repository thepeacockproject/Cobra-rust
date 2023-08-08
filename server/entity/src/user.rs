//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, unique)]
    pub id: u64,
    pub wallet: i32,
    #[sea_orm(column_name = "contractPlays")]
    pub contract_plays: i32,
    pub trophies: i32,
    #[sea_orm(column_name = "competitionPlays")]
    pub competition_plays: i32,
    pub country: i32,
    #[sea_orm(column_name = "displayName")]
    pub display_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::competition::Entity")]
    Competition,
    #[sea_orm(has_many = "super::contract::Entity")]
    Contract,
    #[sea_orm(has_many = "super::queue::Entity")]
    Queue,
    #[sea_orm(has_many = "super::sniperscore::Entity")]
    Sniperscore,
    #[sea_orm(has_many = "super::storyscore::Entity")]
    Storyscore,
    #[sea_orm(has_many = "super::tutorialscore::Entity")]
    Tutorialscore,
}

impl Related<super::queue::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Queue.def()
    }
}

impl Related<super::sniperscore::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sniperscore.def()
    }
}

impl Related<super::storyscore::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Storyscore.def()
    }
}

impl Related<super::tutorialscore::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tutorialscore.def()
    }
}

impl Related<super::competition::Entity> for Entity {
    fn to() -> RelationDef {
        super::competitionscore::Relation::Competition.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::competitionscore::Relation::User.def().rev())
    }
}

impl Related<super::contract::Entity> for Entity {
    fn to() -> RelationDef {
        super::contractscore::Relation::Contract.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::contractscore::Relation::User.def().rev())
    }
}

pub struct RatingLink;

impl Linked for RatingLink {
    type FromEntity = Entity;

    type ToEntity = super::rating::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            super::rating::Relation::User.def().rev(),
            super::rating::Relation::Contract.def(),
        ]
    }
}

pub struct PlayLink;

impl Linked for PlayLink {
    type FromEntity = Entity;

    type ToEntity = super::play::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            super::play::Relation::User.def().rev(),
            super::play::Relation::Contract.def(),
        ]
    }
}

impl ActiveModelBehavior for ActiveModel {}
