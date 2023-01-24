use axum::Json;
use axum::{http::StatusCode, response::IntoResponse, Extension};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

use crate::database::champion;
use crate::database::champion_star;
use crate::errors::AppError;

#[derive(Clone, Serialize)]
pub struct Champion {
    id: String,
    name: String,
    cost: i32,
    mana: i32,
    starting_mana: i32,
    armor: i32,
    magic_resist: i32,
    attack_speed: f32,
    crit_rate: i32,
    range: i32,
    damage: Vec<i32>,
    health: Vec<i32>,
}

impl Champion {
    fn new(basic: &champion::Model, stars: &Vec<champion_star::Model>) -> Self {
        let (damage, health) =
            stars
                .iter()
                .fold((vec![], vec![]), |(mut damage, mut health), star| {
                    damage.push(star.damage);
                    health.push(star.health);
                    (damage, health)
                });
        Self {
            damage,
            health,
            id: basic.id.clone(),
            name: basic.name.clone(),
            cost: basic.cost,
            mana: basic.mana,
            starting_mana: basic.starting_mana,
            armor: basic.armor,
            magic_resist: basic.magic_resist,
            attack_speed: basic.attack_speed,
            crit_rate: basic.crit_rate,
            range: basic.range,
        }
    }
}

pub async fn get_champions(
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<Vec<Champion>>, AppError> {
    Ok(Json(
        champion::Entity::find()
            .find_with_related(champion_star::Entity)
            .all(&database)
            .await?
            .iter()
            .map(|(champion, stars)| Champion::new(champion, stars))
            .collect::<Vec<Champion>>(),
    ))
}
