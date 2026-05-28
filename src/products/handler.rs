use axum::{Json, Router, extract::State, http::StatusCode, middleware::from_fn, routing::post};
use diesel::prelude::Insertable;
use serde::Deserialize;
use validator::Validate;

use super::{
    json_model::{ProductLocalization, ProductOption},
    model::Product,
};
use crate::{
    db::DbPool,
    middlewares::{AxumResponse, BetterAuth, JsonResponse},
    schema,
};

#[derive(Deserialize, Debug, Validate)]
pub(super) struct CreateProductSchema {
    #[allow(dead_code)]
    created_by_id: uuid::Uuid,

    price: i64,

    title: ProductLocalization,
    description: ProductLocalization,

    #[allow(dead_code)]
    options: Option<Vec<ProductOption>>,

    #[validate(length(min = 1, message = "At least one image is required"))]
    image_urls: Vec<String>,
}

impl CreateProductSchema {
    fn to_new_product(self) -> NewProduct {
        let default_json_object = serde_json::Value::Object(serde_json::Map::new());
        let default_json_array = serde_json::Value::Array(Vec::new());
        let product_options = match self.options {
            Some(options) => {
                Some(serde_json::to_value(options).unwrap_or(default_json_array.clone()))
            }
            None => None,
        };
        NewProduct {
            created_by_id: self.created_by_id,
            // Price should times 100 to handle floating numbers
            price: self.price * 100,
            title: serde_json::to_value(self.title).unwrap_or(default_json_object.clone()),
            description: serde_json::to_value(self.description).unwrap_or(default_json_object),
            options: product_options,
            image_urls: serde_json::to_value(self.image_urls).unwrap_or(default_json_array),
        }
    }
}

#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = schema::products)]
pub(super) struct NewProduct {
    created_by_id: uuid::Uuid,
    price: i64,
    title: serde_json::Value,
    description: serde_json::Value,
    options: Option<serde_json::Value>,
    image_urls: serde_json::Value,
}

async fn create_product(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateProductSchema>,
) -> AxumResponse<Product> {
    if let Err(errors) = payload.validate() {
        let error_message = format!("Validation failed: {:?}", errors.0);
        return JsonResponse::send(StatusCode::BAD_REQUEST, None, Some(error_message));
    }
    let new_product = payload.to_new_product();
    match Product::create(&pool, &new_product).await {
        Ok(product) => JsonResponse::send(StatusCode::CREATED, Some(product), None),
        Err(err) => JsonResponse::send(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Some(err.to_string()),
        ),
    }
}

pub fn routes() -> Router<DbPool> {
    Router::new()
        .route("/", post(create_product))
        .layer(from_fn(BetterAuth::admin_middleware))
}
