-- Add up migration script here
-- 1. update_at を自動更新する関数の作成
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS '
    BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
    END;
' LANGUAGE plpgsql;

-- 2. manga テーブルの作成
CREATE TABLE IF NOT EXISTS manga (
    manga_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    short_title VARCHAR(100) UNIQUE NOT NULL,
    url text NOT NULL,
    episode VARCHAR(100),
    portal_kind VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

-- 3. manga テーブルへのトリガー追加
CREATE TRIGGER set_manga_updated_at
    BEFORE UPDATE ON manga
    FOR EACH ROW
    EXECUTE PROCEDURE set_updated_at();