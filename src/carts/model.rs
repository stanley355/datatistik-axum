use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, Queryable};
use diesel_async::RunQueryDsl;
use serde::Serialize;

use crate::{
    carts::handler::CreateCartSchema,
    db::{DbPool, DbPoolExt},
    schema,
};

#[derive(Serialize, Queryable)]
#[diesel(table_name = schema::carts)]
pub(super) struct Cart {
    pub id: uuid::Uuid,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    pub user_id: uuid::Uuid,
    product_id: i32,
    pub(super) amount: i32,
}

impl DbPoolExt for Cart {}
impl Cart {
    pub(super) async fn create(pool: &DbPool, payload: &CreateCartSchema) -> QueryResult<Self> {
        let mut conn = match pool.get().await {
            Ok(connection) => connection,
            Err(e) => {
                return Err(Self::deadpool_to_diesel_error(e));
            }
        };
        diesel::insert_into(schema::carts::table)
            .values(payload)
            .get_result(&mut conn)
            .await
    }

    pub(super) async fn find_by_cart_id(pool: &DbPool, cart_id: &uuid::Uuid) -> QueryResult<Self> {
        let mut conn = match pool.get().await {
            Ok(connection) => connection,
            Err(e) => {
                return Err(Self::deadpool_to_diesel_error(e));
            }
        };

        schema::carts::table
            .find(&cart_id)
            .get_result(&mut conn)
            .await
    }

    pub(super) async fn find_by_user_and_product(
        pool: &DbPool,
        user_id: &uuid::Uuid,
        product_id: &i32,
    ) -> QueryResult<Self> {
        let mut conn = match pool.get().await {
            Ok(connection) => connection,
            Err(e) => {
                return Err(Self::deadpool_to_diesel_error(e));
            }
        };

        schema::carts::table
            .filter(
                schema::carts::user_id
                    .eq(user_id)
                    .and(schema::carts::product_id.eq(product_id)),
            )
            .get_result(&mut conn)
            .await
    }

    pub(super) async fn update_amount_by_cart_id(
        pool: &DbPool,
        cart_id: &uuid::Uuid,
        amount: &i32,
    ) -> QueryResult<Self> {
        let mut conn = match pool.get().await {
            Ok(connection) => connection,
            Err(e) => {
                return Err(Self::deadpool_to_diesel_error(e));
            }
        };

        diesel::update(schema::carts::table.find(cart_id))
            .set(schema::carts::amount.eq(amount))
            .get_result::<Self>(&mut conn)
            .await
    }
}
