use std::str::FromStr;

use serde::Deserialize;

// sqlx::Type は infra 層の都合なのでちょっともれちゃってる
// Deserialize もルーター部分で Axum がパスから取り出すときに使うためのものなので、
// 本当は server 層に起きたい(それだけのために型を分ける手間をさぼっている)
#[derive(Debug, Clone, PartialEq, Eq, Hash, sqlx::Type, Deserialize)]
#[serde(transparent)] // 中身(uuid)としてそのままデシリアライズされるようになる
pub struct MangaId(uuid::Uuid);

impl MangaId {
    pub fn new() -> Self {
        MangaId(uuid::Uuid::new_v4())
    }

    pub fn inner(self) -> uuid::Uuid {
        self.0
    }
}

impl Default for MangaId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for MangaId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .as_simple()
                .encode_lower(&mut uuid::Uuid::encode_buffer())
        )
    }
}

impl FromStr for MangaId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = uuid::Uuid::from_str(s)?;
        Ok(MangaId(uuid))
    }
}

impl From<uuid::Uuid> for MangaId {
    fn from(uuid: uuid::Uuid) -> Self {
        MangaId(uuid)
    }
}
