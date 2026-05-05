// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "currency"))]
    pub struct Currency;
}

diesel::table! {
    account (id) {
        id -> Text,
        accountId -> Text,
        providerId -> Text,
        userId -> Text,
        accessToken -> Nullable<Text>,
        refreshToken -> Nullable<Text>,
        idToken -> Nullable<Text>,
        accessTokenExpiresAt -> Nullable<Timestamptz>,
        refreshTokenExpiresAt -> Nullable<Timestamptz>,
        scope -> Nullable<Text>,
        password -> Nullable<Text>,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Currency;

    products (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        slug -> Varchar,
        name -> Varchar,
        description -> Varchar,
        currency -> Currency,
        price -> Int8,
        unit -> Nullable<Varchar>,
        images -> Jsonb,
        metadata -> Jsonb,
    }
}

diesel::table! {
    session (id) {
        id -> Text,
        expiresAt -> Timestamptz,
        token -> Text,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
        ipAddress -> Nullable<Text>,
        userAgent -> Nullable<Text>,
        userId -> Text,
        impersonatedBy -> Nullable<Text>,
    }
}

diesel::table! {
    user (id) {
        id -> Text,
        name -> Text,
        email -> Text,
        emailVerified -> Bool,
        image -> Nullable<Text>,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
        role -> Nullable<Text>,
        banned -> Nullable<Bool>,
        banReason -> Nullable<Text>,
        banExpires -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    verification (id) {
        id -> Text,
        identifier -> Text,
        value -> Text,
        expiresAt -> Timestamptz,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
    }
}

diesel::joinable!(account -> user (userId));
diesel::joinable!(session -> user (userId));

diesel::allow_tables_to_appear_in_same_query!(account, products, session, user, verification,);
