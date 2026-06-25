use std::str::FromStr;

use super::model::Cart;
use crate::{
    db::DbPool,
    middlewares::{AxumResponse, BetterAuth, JsonResponse},
    schema,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware::from_fn,
    routing::{get, post, put},
};
use diesel::prelude::{AsChangeset, Insertable};
use serde::Deserialize;

#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = schema::carts)]
pub(super) struct CreateCartSchema {
    user_id: uuid::Uuid,
    product_id: i32,
    amount: i32,
}

async fn create_cart(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateCartSchema>,
) -> AxumResponse<Cart> {
    if let Ok(prev_cart) =
        Cart::find_by_user_and_product(&pool, &payload.user_id, &payload.product_id).await
    {
        let new_amount = &prev_cart.amount + &payload.amount;
        return match Cart::update_amount_by_cart_id(&pool, &prev_cart.id, &new_amount).await {
            Ok(product) => JsonResponse::send(StatusCode::CREATED, Some(product), None),
            Err(err) => JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            ),
        };
    }

    match Cart::create(&pool, &payload).await {
        Ok(product) => JsonResponse::send(StatusCode::CREATED, Some(product), None),
        Err(err) => JsonResponse::send(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Some(err.to_string()),
        ),
    }
}

#[derive(Deserialize, AsChangeset, Debug)]
#[diesel(table_name = schema::carts)]
pub(super) struct UpdateCartSchema {
    amount: i32,
}

async fn update_cart(
    State(pool): State<DbPool>,
    Path(cart_id): Path<String>,
    Json(payload): Json<UpdateCartSchema>,
) -> AxumResponse<Cart> {
    let cart_id = match uuid::Uuid::from_str(&cart_id) {
        Ok(id) => id,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            );
        }
    };
    let cart = match Cart::find_by_cart_id(&pool, &cart_id).await {
        Ok(cart) => cart,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            );
        }
    };

    match Cart::update_amount_by_cart_id(&pool, &cart.id, &payload.amount).await {
        Ok(product) => JsonResponse::send(StatusCode::OK, Some(product), None),
        Err(err) => JsonResponse::send(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Some(err.to_string()),
        ),
    }
}

pub fn routes() -> Router<DbPool> {
    Router::new()
        .route("/", post(create_cart))
        .route("/{id}", put(update_cart))
        .layer(from_fn(BetterAuth::session_middleware))
}
