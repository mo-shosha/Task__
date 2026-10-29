
--! create_product
INSERT INTO products (name, price, description, category_id)
VALUES (:name, :price, :description, :category_id)
RETURNING id, name, price, description, category_id;

--! get_products
SELECT id, name, price, description, category_id
FROM products
ORDER BY id;

--! get_product
SELECT id, name, price, description, category_id
FROM products
WHERE id = :id;

--! update_product
UPDATE products
SET name = :name,
    price = :price,
    description = :description,
    category_id = :category_id
WHERE id = :id
RETURNING id, name, price, description, category_id;

--! delete_product
DELETE FROM products
WHERE id = :id
RETURNING id;
