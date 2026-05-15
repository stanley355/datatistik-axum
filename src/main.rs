mod db;
mod enumerates;
mod envs;
mod middlewares;
mod schema;
mod user_search;

use axum::{
    Router,
    http::{HeaderValue, Method},
};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let app_env = envs::Envs::app_env();
    let trusted_domains = envs::Envs::trusted_domains();
    let cors = match app_env.as_str() {
        "production" => CorsLayer::new()
            .allow_origin(trusted_domains.parse::<HeaderValue>().unwrap())
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PATCH,
                Method::PUT,
                Method::DELETE,
            ])
            .allow_headers([
                axum::http::header::CONTENT_TYPE,
                axum::http::header::AUTHORIZATION,
            ]),
        _ => CorsLayer::permissive(),
    };

    let pool = db::build_db_pool().await;

    // build our application with a single route
    let app = Router::new()
        .nest("/user-search", user_search::routes())
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
