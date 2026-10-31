
--! create_image
INSERT INTO images (product_id, file_path, is_primary, display_order)
VALUES (:product_id, :file_path, :is_primary, :display_order)
RETURNING id, product_id, file_path, is_primary, display_order;

--! get_product_images
SELECT id, product_id, file_path, is_primary, display_order
FROM images
WHERE product_id = :product_id
ORDER BY display_order, id;

--! get_image
SELECT id, product_id, file_path, is_primary, display_order
FROM images
WHERE id = :id;

--! update_image
UPDATE images
SET file_path = :file_path,
    is_primary = :is_primary,
    display_order = :display_order
WHERE id = :id
RETURNING id, product_id, file_path, is_primary, display_order;

--! clear_primary_image
UPDATE images
SET is_primary = FALSE
WHERE product_id = :product_id
  AND is_primary;

--! delete_image
DELETE FROM images
WHERE id = :id
RETURNING id;
