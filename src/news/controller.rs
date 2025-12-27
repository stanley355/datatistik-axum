use axum::response::Json;
use axum::{extract::State, http::StatusCode};

use crate::{db::DbPool, news::model::News};
pub(super) async fn find_news(
    State(pool): State<DbPool>,
) -> Result<Json<Vec<News>>, (StatusCode, String)> {
    match News::find(&pool).await {
        Ok(news) => Ok(Json(news)),
        Err(err) => Ok(Json(vec![])),
    }
}
