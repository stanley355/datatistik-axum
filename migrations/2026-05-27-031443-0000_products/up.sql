-- Your SQL goes here
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by_id UUID NOT NULL REFERENCES users(id),
    is_available BOOL NOT NULL DEFAULT true,
    price BIGINT NOT NULL,
    title JSONB NOT NULL DEFAULT '{}',
    description JSONB NOT NULL DEFAULT '{}',
    options JSONB NOT NULL DEFAULT '[]',
    image_urls JSONB NOT NULL DEFAULT '[]',
    source_url VARCHAR
);

-- Register the table with Diesel's internal trigger helper
SELECT diesel_manage_updated_at('products');
