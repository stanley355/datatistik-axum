use diesel::{ExpressionMethods, QueryDsl, QueryResult, Queryable};
use diesel_async::RunQueryDsl;
use serde::Serialize;

use crate::{
    db::{DbPool, DbPoolExt},
    schema,
    user_search::handler::CreateUserSearchSchema,
};

#[derive(Serialize, Queryable)]
#[diesel(table_name = schema::user_search)]
pub(super) struct UserSearch {
    pub id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub keyword: String,
    pub user_id: Option<uuid::Uuid>,
}

impl DbPoolExt for UserSearch {}
impl UserSearch {
    pub(super) async fn create(
        pool: &DbPool,
        payload: &CreateUserSearchSchema,
    ) -> QueryResult<UserSearch> {
        let mut conn = match pool.get().await {
            Ok(connection) => connection,
            Err(e) => {
                return Err(Self::deadpool_to_diesel_error(e));
            }
        };
        println!("Conn success!");
        diesel::insert_into(schema::users_search::table)
            .values(payload)
            .get_result(&mut conn)
            .await
    }
}
