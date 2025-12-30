use crate::middlewares::{AxumResponse, DataPagination, JsonResponse, Pagination};
use crate::websites::WebsiteCategory;
use crate::{db::DbPool, news::model::News};

use axum::extract::Query;
use axum::{extract::State, http::StatusCode};
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub(super) struct FindManyNewsQuery {
    pub(super) website_code: Option<String>,
    pub(super) category: Option<WebsiteCategory>,
    pub(super) is_headline: Option<bool>,
    pub(super) page: Option<u32>,
    pub(super) per_page: Option<u32>,
}

pub(super) async fn find_many_news(
    State(pool): State<DbPool>,
    Query(query): Query<FindManyNewsQuery>,
) -> AxumResponse<DataPagination<Vec<News>>> {
    let news = match News::find_many(&pool, &query).await {
        Ok(news) => news,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            );
        }
    };

    let count = match News::find_many_count(&pool, &query).await {
        Ok(count) => count,
        Err(err) => {
            return JsonResponse::send(
                StatusCode::INTERNAL_SERVER_ERROR,
                None,
                Some(err.to_string()),
            );
        }
    };

    let pagination = Pagination::new(query.page, query.per_page, count as u32);
    let data_pagination = DataPagination::new(Some(news), pagination);
    JsonResponse::send(StatusCode::OK, Some(data_pagination), None)
}
