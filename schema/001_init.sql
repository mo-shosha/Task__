CREATE TABLE categories (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE products (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL,
    price BIGINT NOT NULL CHECK (price >= 0),
    description TEXT,
    category_id BIGINT NOT NULL REFERENCES categories(id) ON DELETE RESTRICT
);

CREATE INDEX idx_products_category_id
    ON products(category_id);
