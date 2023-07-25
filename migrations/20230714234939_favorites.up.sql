-- Add up migration script here
CREATE TABLE IF NOT EXISTS favorites (
    id CHAR(36) PRIMARY KEY NOT NULL,
    show_id CHAR(36) NOT NULL,
    user_id INT UNSIGNED NOT NULL
)
