use super::json_model::CartProductOption;
use super::model::Cart;
use crate::{
    db::DbPool,
    middlewares::{AxumResponse, BetterAuth, DataPagination, JsonResponse, Pagination},
    products::Product,
    schema,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware::from_fn,
    routing::{delete, get, post, put},
};
use diesel::prelude::{AsChangeset, Insertable};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
pub(super) struct CreateCartSchema {
    user_id: uuid::Uuid,
    product_id: i32,
    options: Option<Vec<CartProductOption>>,
    amount: i32,
}

impl CreateCartSchema {
    fn to_new_cart(self) -> NewCart {
        let default_json_array = serde_json::Value::Array(Vec::new());
        NewCart {
            user_id: self.user_id,
            product_id: self.product_id,
            options: serde_json::to_value(self.options).unwrap_or(default_json_array),
            amount: self.amount,
        }
    }
}

#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = schema::carts)]
pub(super) struct NewCart {
    user_id: uuid::Uuid,
    product_id: i32,
    pub(super) options: serde_json::Value,
    pub(super) amount: i32,
}

async fn create_cart(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateCartSchema>,
) -> AxumResponse<Cart> {
    if let Ok(prev_cart) =
        Cart::find_by_user_and_product(&pool, &payload.user_id, &payload.product_id).await
    {
        return match Cart::update_created_cart(&pool, &prev_cart.id, &payload.to_new_cart()).await {
            Ok(product) => JsonResponse::send(StatusCode::CREATED, Some(product), None),
            Err(err) => JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            ),
        };
    }

    match Cart::create(&pool, &payload.to_new_cart()).await {
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

async fn find_cart_by_user(
    State(pool): State<DbPool>,
    Path(user_id): Path<String>,
) -> AxumResponse<DataPagination<Vec<(Cart, Product)>>> {
    let user_uuid = match uuid::Uuid::from_str(&user_id) {
        Ok(id) => id,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            );
        }
    };

    let carts = match Cart::find_by_user_join_product(&pool, &user_uuid).await {
        Ok(data) => data,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            );
        }
    };

    let cart_count = match Cart::count(&pool, &user_uuid).await {
        Ok(data) => data,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            );
        }
    };

    let pagination = Pagination::new(None, None, cart_count as u32);
    let data_pagination = DataPagination::new(Some(carts), pagination);
    JsonResponse::send(StatusCode::OK, Some(data_pagination), None)
}

async fn remove_cart(
    State(pool): State<DbPool>,
    Path(cart_id): Path<String>,
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

    match Cart::remove(&pool, &cart_id).await {
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
        .route("/user/{user_id}", get(find_cart_by_user))
        .route("/{cart_id}", put(update_cart))
        .route("/{cart_id}", delete(remove_cart))
        .layer(from_fn(BetterAuth::session_middleware))
}
