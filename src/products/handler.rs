use aws_sdk_s3::primitives::ByteStream;
use axum::{
    Json, Router,
    body::Body,
    extract::{Multipart, Path, State},
    http::StatusCode,
    middleware::from_fn,
    routing::{get, post},
};
use diesel::prelude::Insertable;
use serde::Deserialize;
use validator::Validate;

use super::{
    json_model::{ProductLocalization, ProductOption},
    model::Product,
};
use crate::{
    db::DbPool,
    envs::Envs,
    middlewares::{AxumResponse, BetterAuth, DataPagination, JsonResponse, Pagination},
    s3::S3,
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

async fn find_product(State(pool): State<DbPool>) -> AxumResponse<DataPagination<Vec<Product>>> {
    let products = match Product::find(&pool).await {
        Ok(data) => data,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            );
        }
    };

    let product_count = match Product::count(&pool).await {
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

async fn upload_product_images(
    State(pool): State<DbPool>,
    Path(id): Path<i32>,
    mut multipart: Multipart,
) -> AxumResponse<String> {
    let endpoint = Envs::s3_endpoint();
    let bucket = Envs::s3_bucket();
    while let Ok(Some(field)) = multipart.next_field().await {
        // 1. EXTRACT METADATA FOR VALIDATION
        let content_type = field.content_type().unwrap_or("").to_string();
        let file_name = field.file_name().unwrap_or("").to_string().to_lowercase();

        // 2. RUN CONTENT-TYPE VALIDATION
        // Valid MIME types: image/png, image/jpeg, image/webp
        let is_valid_mime = matches!(
            content_type.as_str(),
            "image/png" | "image/jpeg" | "image/jpg" | "image/webp"
        );

        // 3. RUN FILE EXTENSION VALIDATION
        let is_valid_ext = file_name.ends_with(".png")
            || file_name.ends_with(".jpeg")
            || file_name.ends_with(".jpg")
            || file_name.ends_with(".webp");

        // Reject if either validation fails
        if !is_valid_mime && !is_valid_ext {
            return JsonResponse::send(
                StatusCode::BAD_REQUEST,
                None,
                Some("Invalid file format. Only PNG, JPEG, JPG, and WEBP are allowed.".to_string()),
            );
        }
        let extension = if file_name.ends_with(".png") {
            "png"
        } else if file_name.ends_with(".webp") {
            "webp"
        } else {
            "jpg"
        };
        // Generate a unique S3 Key to prevent file overwrites
        let unique_id = uuid::Uuid::new_v4();
        let s3_key = format!("{}.{}", unique_id, extension);
        let bytes = field.bytes().await.unwrap();
        let byte_stream = ByteStream::from(bytes);
        let file_upload = S3::upload_file(&s3_key, byte_stream, &content_type).await;
        match file_upload {
            Ok(_) => {
                let public_url = format!("{}/{}/{}", endpoint, bucket, s3_key);
                println!("{}", public_url);
            }
            Err(err) => {
                return JsonResponse::send(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    None,
                    Some(err.to_string()),
                );
            }
        }
    }

    JsonResponse::send(StatusCode::OK, None, None)
}

pub fn routes() -> Router<DbPool> {
    let public_routes = Router::new().route("/", get(find_product));

    let protected_routes = Router::new()
        .route("/", post(create_product))
        .route("/{id}/images", post(upload_product_images))
        .layer(from_fn(BetterAuth::admin_middleware));

    public_routes.merge(protected_routes)
}
