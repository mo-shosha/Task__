--! create_product
INSERT INTO products (name, price, description, category_id)
VALUES (:name, :price, :description, :category_id)
RETURNING id, name, price, description, category_id;

--! get_products (search?, category_id?, min_price?, max_price?) : (primary_image_path?)
SELECT 
  p.id, p.name, p.price, p.description, p.category_id, c.name AS category_name, i.file_path AS primary_image_path
FROM products p
JOIN categories c
    ON p.category_id = c.id
LEFT JOIN images i
    ON p.id = i.product_id
    AND i.is_primary = TRUE
WHERE (:search::TEXT IS NULL
        OR p.name ILIKE '%' || :search || '%'
        OR p.description ILIKE '%' || :search || '%')
  AND (:category_id::BIGINT IS NULL OR p.category_id = :category_id)
  AND (:min_price::BIGINT IS NULL OR p.price >= :min_price)
  AND (:max_price::BIGINT IS NULL OR p.price <= :max_price)
ORDER BY p.id
LIMIT :limit
OFFSET :offset;

--! count_products (search?, category_id?, min_price?, max_price?)
SELECT COUNT(*) AS total
FROM products p
WHERE (:search::TEXT IS NULL
        OR p.name ILIKE '%' || :search || '%'
        OR p.description ILIKE '%' || :search || '%')
  AND (:category_id::BIGINT IS NULL OR p.category_id = :category_id)
  AND (:min_price::BIGINT IS NULL OR p.price >= :min_price)
  AND (:max_price::BIGINT IS NULL OR p.price <= :max_price);

--! get_product : (image_id?, file_path?, is_primary?, display_order?)
SELECT
    p.id, p.name, p.price, p.description, p.category_id, i.id AS image_id, i.file_path, i.is_primary, i.display_order
FROM products p
LEFT JOIN images i
    ON p.id = i.product_id
WHERE p.id = :id
ORDER BY i.display_order, i.id;

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
