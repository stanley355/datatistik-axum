use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable};
use diesel_async::RunQueryDsl;
use serde::Serialize;

use crate::{
    db::{DbPool, DbPoolExt},
    products::handler::{NewProduct, UpdateProduct},
    schema,
};

#[derive(Serialize, Queryable)]
#[diesel(table_name = schema::products)]
pub(super) struct Product {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub created_by_id: uuid::Uuid,
    pub is_available: bool,
    price: i64,
    title: serde_json::Value,
    description: serde_json::Value,
    options: serde_json::Value,
    image_urls: serde_json::Value,
    image_cover_number: i32,
    source_url: Option<String>,
}

impl DbPoolExt for Product {}
impl Product {
    pub(super) async fn create(pool: &DbPool, payload: &NewProduct) -> QueryResult<Self> {
        let mut conn = match pool.get().await {
            Ok(connection) => connection,
            Err(e) => {
                return Err(Self::deadpool_to_diesel_error(e));
            }
        };
        diesel::insert_into(schema::products::table)
            .values(payload)
            .get_result(&mut conn)
            .await
    }

    pub(super) async fn find(pool: &DbPool) -> QueryResult<Vec<Self>> {
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

    pub(super) async fn count(pool: &DbPool) -> QueryResult<i64> {
        let mut conn = match pool.get().await {
            Ok(connection) => connection,
            Err(e) => {
                return Err(Self::deadpool_to_diesel_error(e));
            }
        };
        schema::products::table.count().get_result(&mut conn).await
    }

    pub(super) async fn find_by_id(pool: &DbPool, id: &i32) -> QueryResult<Self> {
        let mut conn = match pool.get().await {
            Ok(connection) => connection,
            Err(e) => {
                return Err(Self::deadpool_to_diesel_error(e));
            }
        };
        schema::products::table
            .filter(schema::products::id.eq(id))
            .get_result(&mut conn)
            .await
    }

    pub async fn update(
        pool: &DbPool,
        product_id: &i32,
        changes: &UpdateProduct,
    ) -> QueryResult<Self> {
        let mut conn = match pool.get().await {
            Ok(connection) => connection,
            Err(e) => {
                return Err(Self::deadpool_to_diesel_error(e));
            }
        };

        diesel::update(schema::products::table.find(product_id))
            .set(changes)
            .get_result::<Product>(&mut conn)
            .await
    }
}
