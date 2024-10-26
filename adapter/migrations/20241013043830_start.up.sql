-- Add up migration script here
-- 1. updated_at を自動更新する関数の作成
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS '
  BEGIN
    new.updated_at := ''now'';
    return new;
  END;
' LANGUAGE 'plpgsql';

-- 2. mangas テーブルの作成
CREATE TABLE IF NOT EXISTS mangas (
    manga_id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    short_title VARCHAR(255) NOT NULL UNIQUE,
    episode VARCHAR(255) NOT NULL DEFAULT '初期状態',
    url VARCHAR(255) NOT NULL DEFAULT '初期状態',
    created_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

-- 3. mangas テーブルへのトリガーの追加
CREATE TRIGGER mangas_updated_at_trigger
    BEFORE UPDATE ON mangas FOR EACH ROW
    EXECUTE PROCEDURE set_updated_at();
