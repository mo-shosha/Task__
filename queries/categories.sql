
--! create_category
INSERT INTO categories (name, description)
VALUES (:name, :description)
RETURNING id, name, description;

--! get_categories
SELECT id, name, description
FROM categories
ORDER BY id;

--! get_category
SELECT id, name, description
FROM categories
WHERE id = :id;

--! update_category
UPDATE categories
SET name = :name,
    description = :description
WHERE id = :id
RETURNING id, name, description;

--! delete_category
DELETE FROM categories
WHERE id = :id
RETURNING id;
