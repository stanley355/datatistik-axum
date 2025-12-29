mod db;
mod envs;
mod middlewares;
mod news;
mod schema;
mod websites;

use axum::{Router, http::HeaderValue};
use tower_http::cors::CorsLayer;

use crate::news::news_routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let app_env = envs::Envs::app_env();
    let cors = match app_env.as_str() {
        "production" => {
            CorsLayer::new().allow_origin("https://datatistik.com".parse::<HeaderValue>().unwrap())
        }
        _ => CorsLayer::permissive(),
    };

    let pool = db::build_db_pool().await;

    // build our application with a single route
    let app = Router::new()
        .nest("/news", news_routes())
        .with_state(pool)
        .layer(cors);

    // run our app with hyper, listening globally on port 8000
    let host_address = envs::Envs::host_address();
    let listener = match tokio::net::TcpListener::bind(&host_address).await {
        Ok(listen) => listen,
        Err(err) => panic!("Failed to bind to address: {}", err),
    };

    println!("Server started at {}", &host_address);
    match axum::serve(listener, app).await {
        Ok(_) => {}
        Err(err) => panic!("Failed to start server: {}", err),
    }
}
