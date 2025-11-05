-- Add down migration script here
DROP TRIGGER IF EXISTS set_manga_updated_at ON manga;
DROP TABLE IF EXISTS manga;

DROP FUNCTION IF EXISTS set_updated_at;