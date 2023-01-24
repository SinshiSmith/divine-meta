use axum::Json;
use axum::{http::StatusCode, response::IntoResponse, Extension};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

use crate::database::champion;
use crate::database::champion_star;
use crate::errors::AppError;

#[derive(Clone, Serialize)]
pub struct ResponseChampion {
    id: String,
    name: String,
    cost: i32,
    damage: Vec<i32>,
}

pub async fn get_champions(
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<Vec<ResponseChampion>>, AppError> {
    Ok(Json(
        champion::Entity::find()
            .find_with_related(champion_star::Entity)
            .all(&database)
            .await?
            .iter()
            .map(|(champion, stars)| ResponseChampion {
                id: champion.id.clone(),
                name: champion.name.clone(),
                cost: champion.cost,
                damage: stars.iter().map(|star| star.damage).collect(),
            })
            .collect::<Vec<ResponseChampion>>(),
    ))
}
