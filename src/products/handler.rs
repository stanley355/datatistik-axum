use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    middleware::from_fn,
    routing::{get, post, put},
};
use diesel::prelude::{AsChangeset, Insertable};
use serde::Deserialize;
use validator::Validate;

use super::{
    json_model::{ProductLocalization, ProductOption},
    model::Product,
};
use crate::{
    db::DbPool,
    middlewares::{AxumResponse, BetterAuth, DataPagination, JsonResponse, Pagination},
    s3::S3Image,
    schema,
};

#[derive(Deserialize, Debug, Validate)]
pub(super) struct CreateProductSchema {
    #[allow(dead_code)]
    created_by_id: uuid::Uuid,
    price: i64,
    is_available: bool,
    title: ProductLocalization,
    description: ProductLocalization,
    #[allow(dead_code)]
    options: Option<Vec<ProductOption>>,
    #[validate(length(min = 1, message = "At least one image is required"))]
    image_urls: Vec<S3Image>,
    image_cover_number: i32,
    source_url: Option<String>,
}

impl CreateProductSchema {
    fn to_new_product(self) -> NewProduct {
        let default_json_object = serde_json::Value::Object(serde_json::Map::new());
        let default_json_array = serde_json::Value::Array(Vec::new());
        let product_options = match self.options {
            Some(options) => {
                let formatted_options: Vec<ProductOption> = options
                    .into_iter()
                    .map(|opt| opt.format_values_price())
                    .collect();
                Some(serde_json::to_value(formatted_options).unwrap_or(default_json_array.clone()))
            }
            None => None,
        };
        NewProduct {
            created_by_id: self.created_by_id,
            // Price should times 100 to handle floating numbers
            price: self.price * 100,
            is_available: self.is_available,
            title: serde_json::to_value(self.title).unwrap_or(default_json_object.clone()),
            description: serde_json::to_value(self.description).unwrap_or(default_json_object),
            options: product_options,
            image_urls: serde_json::to_value(self.image_urls).unwrap_or(default_json_array),
            image_cover_number: self.image_cover_number,
            source_url: self.source_url,
        }
    }
}

#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = schema::products)]
pub(super) struct NewProduct {
    created_by_id: uuid::Uuid,
    price: i64,
    is_available: bool,
    title: serde_json::Value,
    description: serde_json::Value,
    options: Option<serde_json::Value>,
    image_urls: serde_json::Value,
    image_cover_number: i32,
    source_url: Option<String>,
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

#[derive(Deserialize, Debug)]
pub(super) struct FindProductSchema {
    pub(super) is_available: Option<bool>,
}
async fn find_product(
    State(pool): State<DbPool>,
    Query(query): Query<FindProductSchema>,
) -> AxumResponse<DataPagination<Vec<Product>>> {
    let products = match Product::find(&pool, &query).await {
        Ok(data) => data,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            );
        }
    };

    let product_count = match Product::count(&pool, &query).await {
        Ok(data) => data,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            );
        }
    };

    let pagination = Pagination::new(None, None, product_count as u32);
    let data_pagination = DataPagination::new(Some(products), pagination);
    JsonResponse::send(StatusCode::OK, Some(data_pagination), None)
}

async fn find_product_by_id(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
) -> AxumResponse<Product> {
    match Product::find_by_id(&pool, &id).await {
        Ok(data) => JsonResponse::send(StatusCode::OK, Some(data), None),
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            );
        }
    }
}

#[derive(Deserialize, Debug, Validate)]
pub(super) struct UpdateProductSchema {
    price: i64,
    is_available: bool,

    title: ProductLocalization,
    description: ProductLocalization,

    options: Option<Vec<ProductOption>>,

    #[validate(length(min = 1, message = "At least one image is required"))]
    image_urls: Vec<S3Image>,

    image_cover_number: i32,
    source_url: Option<String>,
}

impl UpdateProductSchema {
    fn to_update_product(self) -> UpdateProduct {
        let default_json_object = serde_json::Value::Object(serde_json::Map::new());
        let default_json_array = serde_json::Value::Array(Vec::new());
        let product_options = match self.options {
            Some(options) => {
                let formatted_options: Vec<ProductOption> = options
                    .into_iter()
                    .map(|opt| opt.format_values_price())
                    .collect();
                Some(serde_json::to_value(formatted_options).unwrap_or(default_json_array.clone()))
            }
            None => None,
        };
        UpdateProduct {
            // Price should times 100 to handle floating numbers
            price: self.price * 100,
            is_available: self.is_available,
            title: serde_json::to_value(self.title).unwrap_or(default_json_object.clone()),
            description: serde_json::to_value(self.description).unwrap_or(default_json_object),
            options: product_options,
            image_urls: serde_json::to_value(self.image_urls).unwrap_or(default_json_array),
            image_cover_number: self.image_cover_number,
            source_url: self.source_url,
        }
    }
}

#[derive(Deserialize, AsChangeset, Debug)]
#[diesel(table_name = schema::products)]
pub(super) struct UpdateProduct {
    price: i64,
    is_available: bool,
    title: serde_json::Value,
    description: serde_json::Value,
    options: Option<serde_json::Value>,
    image_urls: serde_json::Value,
    image_cover_number: i32,
    source_url: Option<String>,
}

async fn update_product(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateProductSchema>,
) -> AxumResponse<Product> {
    if let Err(errors) = payload.validate() {
        let error_message = format!("Validation failed: {:?}", errors.0);
        return JsonResponse::send(StatusCode::BAD_REQUEST, None, Some(error_message));
    }

    let update_data = payload.to_update_product();

    match Product::update(&pool, &id, &update_data).await {
        Ok(product) => JsonResponse::send(StatusCode::OK, Some(product), None),
        Err(err) => JsonResponse::send(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Some(err.to_string()),
        ),
    }
}

pub fn routes() -> Router<DbPool> {
    let public_routes = Router::new()
        .route("/", get(find_product))
        .route("/{id}", get(find_product_by_id));

    let protected_routes = Router::new()
        .route("/", post(create_product))
        .route("/{id}", put(update_product)) // Added PUT update route
        .layer(from_fn(BetterAuth::admin_middleware));

    public_routes.merge(protected_routes)
}

impl Product {}
