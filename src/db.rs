use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, bb8},
};

use crate::envs::Envs;

pub type DbPool = bb8::Pool<AsyncPgConnection>;

pub async fn build_db_pool() -> DbPool {
    let config =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(Envs::database_url());
    match bb8::Pool::builder().build(config).await {
        Ok(pool) => pool,
        Err(err) => panic!("Failed to build database pool: {}", err),
    }
}
