use axum::Json;
use axum::{http::StatusCode, response::IntoResponse, Extension};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

use crate::database::champion;
use crate::database::champion_star;

#[derive(Clone, Serialize)]
pub struct ResponseChampion {
    id: String,
    name: String,
    cost: i32,
    damage: Vec<i32>,
}

pub async fn get_champions(
    Extension(database): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    match champion::Entity::find()
        .find_with_related(champion_star::Entity)
        .all(&database)
        .await
    {
        Ok(champions) => {
            let champions_list = champions
                .iter()
                .map(|(champion, stars)| ResponseChampion {
                    id: champion.id.clone(),
                    name: champion.name.clone(),
                    cost: champion.cost,
                    damage: stars.iter().map(|star| star.damage).collect(),
                })
                .collect::<Vec<ResponseChampion>>();

            (StatusCode::OK, Ok(Json(champions_list)))
        }
        Err(_) => (StatusCode::NOT_FOUND, Err("Error while getting champions")),
    }
}
