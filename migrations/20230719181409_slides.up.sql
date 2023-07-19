-- Add up migration script here

CREATE TABLE IF NOT EXISTS slides (
    id CHAR(36) PRIMARY KEY NOT NULL,
    show_id CHAR(36) NOT NULL,
    owner_id INT UNSIGNED NOT NULL,
    content VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
)