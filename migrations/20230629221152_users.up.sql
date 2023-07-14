-- Add up migration script here
CREATE TABLE
    IF NOT EXISTS users (
        id CHAR(36) PRIMARY KEY NOT NULL,
        email VARCHAR(255) NOT NULL,
        name VARCHAR(255) NOT NULL
    );
