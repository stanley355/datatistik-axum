mod db;
mod envs;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = db::build_db_pool().await;

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(pool);

    // run our app with hyper, listening globally on port 8000
    let host_address = envs::Envs::host_address();
    let listener = match tokio::net::TcpListener::bind(&host_address).await {
        Ok(listen) => listen,
        Err(err) => panic!("Failed to bind to address: {}", err),
    };

    println!("Server started at {}", &host_address);
    match axum::serve(listener, app).await {
        Ok(_) => println!("Server started successfully"),
        Err(err) => panic!("Failed to start server: {}", err),
    }
}
