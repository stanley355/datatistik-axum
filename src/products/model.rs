use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable};
use diesel_async::RunQueryDsl;
use serde::Serialize;

use crate::{
    db::{DbPool, DbPoolExt},
    enumerates::Currency,
    schema,
};

#[derive(Serialize, Queryable)]
#[diesel(table_name = schema::products)]
pub(super) struct Product {
    id: uuid::Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    slug: String,
    name: String,
    description: String,
    currency: Currency,
    pub price: i64,
    unit: Option<String>,
    images: serde_json::Value,
    metadata: serde_json::Value,
}

impl DbPoolExt for Product {}

impl Product {
    pub async fn find(pool: &DbPool) -> QueryResult<Vec<Product>> {
        let mut conn = match pool.get().await {
            Ok(connection) => connection,
            Err(e) => {
                return Err(Self::deadpool_to_diesel_error(e));
            }
        };

        schema::products::table
            .order_by(schema::products::created_at.desc())
            .get_results(&mut conn)
            .await
    }

    pub async fn count(pool: &DbPool) -> QueryResult<i64> {
        let mut conn = match pool.get().await {
            Ok(connection) => connection,
            Err(e) => {
                return Err(Self::deadpool_to_diesel_error(e));
            }
        };

        schema::products::table.count().get_result(&mut conn).await
    }
}
