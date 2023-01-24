mod get_champions;

use axum::{body::Body, routing::get, Extension, Router};
use sea_orm::DatabaseConnection;

use axum;
use get_champions::{get_champions, get_one_champion};

pub fn create_routes(database: DatabaseConnection) -> Router<(), Body> {
    Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/champions", get(get_champions))
        .route("/champions/:id", get(get_one_champion))
        .layer(Extension(database))
}
