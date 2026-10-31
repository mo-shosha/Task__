CREATE TABLE images (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    file_path TEXT NOT NULL,
    is_primary BOOLEAN NOT NULL DEFAULT FALSE,
    display_order INTEGER NOT NULL DEFAULT 0,
    product_id BIGINT NOT NULL
        REFERENCES products(id) ON DELETE CASCADE
);

CREATE INDEX idx_images_product_id
    ON images(product_id);

CREATE UNIQUE INDEX idx_images_one_primary_per_product
    ON images(product_id)
    WHERE is_primary;
