-- Your SQL goes here
CREATE TABLE user_search (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    keyword VARCHAR NOT NULL,
    user_id TEXT REFERENCES "user"(id) ON DELETE CASCADE
);

-- Register the table with Diesel's internal trigger helper
SELECT diesel_manage_updated_at('user_search');
