-- Add down migration script here
DROP TRIGGER IF EXISTS mangas_updated_at_trigger ON mangas;
DROP TABLE IF EXISTS mangas;

DROP FUNCTION set_updated_at;