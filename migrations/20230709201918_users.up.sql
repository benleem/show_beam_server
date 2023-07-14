-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS users (
        id INT UNSIGNED PRIMARY KEY NOT NULL,
        `name` VARCHAR(255) NOT NULL,
        username VARCHAR(255) NOT NULL,
        profile_url VARCHAR(255) NOT NULL,
        avatar_url VARCHAR(255) NOT NULL,
        email VARCHAR(255) NOT NULL,
        create_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
    );