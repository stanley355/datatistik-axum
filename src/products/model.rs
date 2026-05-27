use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable};
use diesel_async::RunQueryDsl;
use serde::Serialize;

use super::handler::CreateProductSchema;
use crate::{
    db::{DbPool, DbPoolExt},
    products::handler::NewProduct,
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
    titles: serde_json::Value,
    descriptions: serde_json::Value,
    options: serde_json::Value,
    images_url: serde_json::Value,
}

impl DbPoolExt for Product {}
impl Product {
    pub(super) async fn create(pool: &DbPool, payload: &NewProduct) -> QueryResult<Product> {
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
}
