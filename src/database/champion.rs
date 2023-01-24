//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "champion")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    pub cost: i32,
    pub mana: i32,
    pub starting_mana: i32,
    pub armor: i32,
    pub magic_resist: i32,
    pub attack_speed: f32,
    pub crit_rate: i32,
    pub range: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::champion_star::Entity")]
    ChampionStar,
}

impl Related<super::champion_star::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChampionStar.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
