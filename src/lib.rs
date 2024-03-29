mod database;
mod errors;
mod routes;

use routes::create_routes;
use sea_orm::Database;

pub async fn app(database_uri: &str) {
    let database = Database::connect(database_uri).await.unwrap();
    let app = create_routes(database);

    axum::Server::bind(&"0.0.0.0:3010".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
