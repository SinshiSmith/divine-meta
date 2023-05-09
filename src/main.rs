use std::env;

use divine_meta::app;

#[tokio::main]
async fn main() {
    let database_uri = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => panic!("Could not find Environment variable:  DATABASE_URL"),
    };

    app(&database_uri).await
}
