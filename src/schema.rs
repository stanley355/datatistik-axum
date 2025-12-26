// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "website_category"))]
    pub struct WebsiteCategory;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WebsiteCategory;

    news (id) {
        id -> Uuid,
        #[max_length = 255]
        website_code -> Varchar,
        category -> WebsiteCategory,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        published_at -> Timestamp,
        slug -> Varchar,
        image_url -> Nullable<Varchar>,
        title -> Varchar,
        content -> Text,
        seo_title -> Nullable<Varchar>,
        seo_description -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::WebsiteCategory;

    websites (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        code -> Varchar,
        category -> WebsiteCategory,
        url -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(news, websites,);
