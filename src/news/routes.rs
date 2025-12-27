use axum::Router;
use axum::routing::get;

use crate::{db::DbPool, news::controller::find_news};

pub fn news_routes() -> Router<DbPool> {
    Router::new().route("/", get(find_news))
}
