use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, Selectable};
use diesel_async::RunQueryDsl;
use serde::Serialize;

use crate::db::{DbPool, DbPoolExt};
use crate::news::controller::FindManyNewsQuery;
use crate::schema;
use crate::websites::WebsiteCategory;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::news)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct News {
    id: uuid::Uuid,
    website_code: String,
    category: WebsiteCategory,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    published_at: chrono::NaiveDateTime,
    slug: String,
    image_url: Option<String>,
    title: String,
    content: String,
    seo_title: Option<String>,
    seo_description: Option<String>,
    is_headline: bool,
}

impl DbPoolExt for News {}

impl News {
    pub async fn find_many(pool: &DbPool, query: &FindManyNewsQuery) -> QueryResult<Vec<News>> {
        let mut conn = match pool.get().await {
            Ok(connection) => connection,
            Err(e) => {
                return Err(Self::deadpool_to_diesel_error(e));
            }
        };
        let mut news_sql_query = schema::news::table
            .order_by(schema::news::published_at.desc())
            .into_boxed();

        if let Some(is_headline) = query.is_headline {
            news_sql_query = news_sql_query.filter(schema::news::is_headline.eq(is_headline));
        }

        news_sql_query.get_results(&mut conn).await
    }
}
