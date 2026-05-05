-- Your SQL goes here
-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE CURRENCY AS ENUM ('RMB', 'IDR');

CREATE TABLE products (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    slug VARCHAR UNIQUE NOT NULL,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    currency CURRENCY NOT NULL,
    price BIGINT NOT NULL,
    unit VARCHAR,
    images JSONB NOT NULL DEFAULT '[]',
    metadata JSONB NOT NULL DEFAULT '{}'
);

SELECT
    diesel_manage_updated_at ('products');
