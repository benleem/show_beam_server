CREATE TABLE
    IF NOT EXISTS shows (
        id CHAR(36) PRIMARY KEY NOT NULL,
        owner CHAR(36) NOT NULL UNIQUE,
        title VARCHAR(255) NOT NULL,
        description TEXT NOT NULL,
        view_code VARCHAR(255) UNIQUE
    );
