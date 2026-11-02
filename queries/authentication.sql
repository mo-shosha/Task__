--! create_user
WITH inserted AS (
    INSERT INTO users (
        username,
        email,
        password_hash,
        role_id
    )
    VALUES (
        :username,
        :email,
        :password_hash,
        (SELECT id FROM roles WHERE name = 'customer')
    )
    RETURNING
        id,
        username,
        email,
        role_id,
        is_active,
        created_at
)
SELECT
    inserted.id,
    inserted.username,
    inserted.email,
    roles.name AS role,
    inserted.is_active,
    inserted.created_at
FROM inserted
JOIN roles ON roles.id = inserted.role_id;


--! get_user_by_email
SELECT
    users.id,
    users.username,
    users.email,
    users.password_hash,
    roles.name AS role,
    users.is_active,
    users.created_at
FROM users
JOIN roles ON roles.id = users.role_id
WHERE users.email = :email;


--! get_user_by_id
SELECT
    users.id,
    users.username,
    users.email,
    roles.name AS role,
    users.is_active,
    users.created_at
FROM users
JOIN roles ON roles.id = users.role_id
WHERE users.id = :id;


--! get_user_by_username
SELECT
    users.id,
    users.username,
    users.email,
    roles.name AS role,
    users.is_active,
    users.created_at
FROM users
JOIN roles ON roles.id = users.role_id
WHERE users.username = :username;


--! create_refresh_token
INSERT INTO refresh_tokens (
    user_id,
    token_hash,
    expires_at
)
VALUES (
    :user_id,
    :token_hash,
    :expires_at
)
RETURNING
    id,
    user_id,
    expires_at,
    created_at;


--! get_refresh_token : (revoked_at?)
SELECT
    id,
    user_id,
    token_hash,
    expires_at,
    revoked_at,
    created_at
FROM refresh_tokens
WHERE token_hash = :token_hash;


--! revoke_refresh_token
UPDATE refresh_tokens
SET revoked_at = NOW()
WHERE token_hash = :token_hash
  AND revoked_at IS NULL;


--! revoke_all_user_refresh_tokens
UPDATE refresh_tokens
SET revoked_at = NOW()
WHERE user_id = :user_id
  AND revoked_at IS NULL;
