-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE website_category AS ENUM ('GOVERNMENT', 'ECONOMY', 'STATISTICS');

CREATE TABLE websites (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    code VARCHAR(255) NOT NULL UNIQUE,
    category website_category NOT NULL,
    url VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    description TEXT
);

CREATE TABLE news (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    website_code VARCHAR(255) NOT NULL REFERENCES websites(code) ON UPDATE CASCADE,
    category website_category NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    published_at TIMESTAMP NOT NULL DEFAULT NOW (),
    slug VARCHAR NOT NULL UNIQUE,
    image_url VARCHAR,
    title VARCHAR NOT NULL,
    content TEXT NOT NULL,
    seo_title VARCHAR,
    seo_description TEXT,
    is_headline BOOLEAN NOT NULL DEFAULT FALSE
);
