use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable, Selectable};
use diesel_async::RunQueryDsl;
use serde::Serialize;

use crate::db::{DbPool, DbPoolExt};
use crate::middlewares::DEFAULT_PER_PAGE;
use crate::news::controller::FindManyNewsQuery;
use crate::schema;
use crate::websites::WebsiteCategory;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = schema::news)]
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

        if let Some(is_headline) = &query.is_headline {
            news_sql_query = news_sql_query.filter(schema::news::is_headline.eq(is_headline));
        }

        if let Some(category) = &query.category {
            news_sql_query = news_sql_query.filter(schema::news::category.eq(category));
        }

        if let Some(website_code) = &query.website_code {
            news_sql_query = news_sql_query.filter(schema::news::website_code.eq(website_code));
        }

        let limit = match &query.per_page {
            Some(limit) => limit.to_owned(),
            None => DEFAULT_PER_PAGE,
        };

        if let Some(page) = &query.page {
            let offset = (page - 1) * limit;
            news_sql_query = news_sql_query.offset(offset as i64).limit(limit as i64);
        }

        news_sql_query
            .limit(limit as i64)
            .get_results(&mut conn)
            .await
    }

    pub async fn find_many_count(pool: &DbPool, query: &FindManyNewsQuery) -> QueryResult<i64> {
        let mut conn = match pool.get().await {
            Ok(connection) => connection,
            Err(e) => {
                return Err(Self::deadpool_to_diesel_error(e));
            }
        };

        let mut news_sql_query = schema::news::table.count().into_boxed();

        if let Some(is_headline) = &query.is_headline {
            news_sql_query = news_sql_query.filter(schema::news::is_headline.eq(is_headline));
        }

        if let Some(category) = &query.category {
            news_sql_query = news_sql_query.filter(schema::news::category.eq(category));
        }

        if let Some(website_code) = &query.website_code {
            news_sql_query = news_sql_query.filter(schema::news::website_code.eq(website_code));
        }

        news_sql_query.get_result(&mut conn).await
    }
}
