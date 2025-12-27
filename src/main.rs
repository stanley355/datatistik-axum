mod db;
mod envs;
mod news;
mod schema;
mod websites;

use axum::Router;

use crate::news::news_routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = db::build_db_pool().await;

    // build our application with a single route
    let app = Router::new().nest("/news", news_routes()).with_state(pool);

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
