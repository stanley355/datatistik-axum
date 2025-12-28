use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, deadpool},
};

use crate::envs::Envs;

pub type DbPool = deadpool::Pool<AsyncPgConnection>;

pub async fn build_db_pool() -> DbPool {
    let config =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(Envs::database_url());
    match deadpool::Pool::builder(config).build() {
        Ok(pool) => pool,
        Err(err) => panic!("Failed to build database pool: {}", err),
    }
}

pub trait DbPoolExt {
    fn deadpool_to_diesel_error(e: deadpool::PoolError) -> diesel::result::Error {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new(e.to_string()),
        )
    }
}
