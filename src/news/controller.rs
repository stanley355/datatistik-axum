use crate::middlewares::{AxumResponse, JsonResponse};
use crate::{db::DbPool, news::model::News};

use axum::extract::Query;
use axum::{extract::State, http::StatusCode};
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub(super) struct FindNewsQuery {
    pub is_headline: Option<bool>,
}

pub(super) async fn find_news(
    State(pool): State<DbPool>,
    Query(query): Query<FindNewsQuery>,
) -> AxumResponse<Vec<News>> {
    match News::find(&pool).await {
        Ok(news) => JsonResponse::send(StatusCode::OK, Some(news), None),
        Err(err) => JsonResponse::send(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Some(err.to_string()),
        ),
    }
}
