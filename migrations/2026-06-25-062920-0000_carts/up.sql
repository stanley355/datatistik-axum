-- Your SQL goes here
CREATE TABLE carts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    options JSONB NOT NULL DEFAULT '[]',
    amount INTEGER NOT NULL DEFAULT 1,

    CONSTRAINT unique_carts_user_product UNIQUE (user_id, product_id)
);

-- Register the table with Diesel's internal trigger helper
SELECT diesel_manage_updated_at('carts');
