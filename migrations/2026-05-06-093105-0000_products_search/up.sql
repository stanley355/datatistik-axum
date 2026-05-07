-- Your SQL goes here
--
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE users_search (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    keyword VARCHAR NOT NULL,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE
);

-- Register the table with Diesel's internal trigger helper
SELECT diesel_manage_updated_at('users_search');
