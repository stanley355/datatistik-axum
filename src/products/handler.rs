use super::model::Product;
use crate::db::DbPool;
use crate::middlewares::{AxumResponse, DataPagination, JsonResponse, Pagination};

use axum::{Router, extract::State, http::StatusCode, routing::get};

pub(super) async fn find_products(
    State(pool): State<DbPool>,
) -> AxumResponse<DataPagination<Vec<Product>>> {
    let news = match Product::find(&pool).await {
        Ok(news) => news,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            );
        }
    };

    let count = match Product::count(&pool).await {
        Ok(count) => count,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            );
        }
    };

    let pagination = Pagination::new(Some(1), Some(100), count as u32);
    let data_pagination = DataPagination::new(Some(news), pagination);
    JsonResponse::send(StatusCode::OK, Some(data_pagination), None)
}

pub fn routes() -> Router<DbPool> {
    Router::new().route("/", get(find_products))
}
