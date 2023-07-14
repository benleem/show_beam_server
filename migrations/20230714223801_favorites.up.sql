-- Add up migration script here
CREATE TABLE IF NOT EXISTS favorites(
    id INT UNSIGNED PRIMARY KEY NOT NULL,
    show_id char(36) UNSIGNED NOT NULL,
    user_id INT UNSIGNED NOT NULL,
);
