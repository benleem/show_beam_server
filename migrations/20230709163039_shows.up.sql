-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS shows (
        id CHAR(36) PRIMARY KEY NOT NULL,
        owner_id CHAR(36) NOT NULL,
        title VARCHAR(255) NOT NULL,
        description TEXT NOT NULL,
        view_code CHAR(36) UNIQUE NULL
    );
