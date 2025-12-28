use crate::middlewares::{AxumResponse, JsonResponse};
use crate::{db::DbPool, news::model::News};
use axum::{extract::State, http::StatusCode};

pub(super) async fn find_news(State(pool): State<DbPool>) -> AxumResponse<Vec<News>> {
    match News::find(&pool).await {
        Ok(news) => JsonResponse::send(StatusCode::OK, Some(news), None),
        Err(err) => JsonResponse::send(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Some(err.to_string()),
        ),
    }
}
