use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use diesel::prelude::Insertable;
use serde::Deserialize;

use super::model::UserSearch;
use crate::{
    db::DbPool,
    middlewares::{AxumResponse, JsonResponse},
    schema,
};

#[derive(Deserialize, Insertable, Debug)]
#[diesel(table_name = schema::users_search)]
pub(super) struct CreateUserSearchSchema {
    keyword: String,
    user_id: Option<uuid::Uuid>,
}

async fn create_user_search(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateUserSearchSchema>,
) -> AxumResponse<UserSearch> {
    match UserSearch::create(&pool, &payload).await {
        Ok(search) => JsonResponse::send(StatusCode::CREATED, Some(search), None),
        Err(err) => JsonResponse::send(
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
            Some(err.to_string()),
        ),
    }
}

pub fn routes() -> Router<DbPool> {
    Router::new().route("/", post(create_user_search))
}
