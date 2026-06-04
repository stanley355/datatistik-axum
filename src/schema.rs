// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Uuid,
        accountId -> Text,
        providerId -> Text,
        userId -> Uuid,
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
    products (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        created_by_id -> Uuid,
        is_available -> Bool,
        price -> Int8,
        title -> Jsonb,
        description -> Jsonb,
        options -> Jsonb,
        image_urls -> Jsonb,
        source_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Uuid,
        expiresAt -> Timestamptz,
        token -> Text,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
        ipAddress -> Nullable<Text>,
        userAgent -> Nullable<Text>,
        userId -> Uuid,
        impersonatedBy -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
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
    users_search (id) {
        id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        keyword -> Varchar,
        user_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    verifications (id) {
        id -> Uuid,
        identifier -> Text,
        value -> Text,
        expiresAt -> Timestamptz,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
    }
}

diesel::joinable!(accounts -> users (userId));
diesel::joinable!(products -> users (created_by_id));
diesel::joinable!(sessions -> users (userId));
diesel::joinable!(users_search -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    products,
    sessions,
    users,
    users_search,
    verifications,
);
