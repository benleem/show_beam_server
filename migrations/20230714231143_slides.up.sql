-- Add up migration script here
CREATE TABLE IF NOT EXISTS slides (
    id INT UNSIGNED PRIMARY KEY NOT NULL,
    show_id CHAR(36) NOT NULL,
    content VARCHAR(255) NOT NULL,
)
