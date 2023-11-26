-- Add down migration script here
BEGIN;
    DROP TABLE IF EXISTS slides;
    DROP TABLE IF EXISTS shows;
    DROP TABLE IF EXISTS favorites;
    DROP TABLE IF EXISTS editors;
COMMIT;

