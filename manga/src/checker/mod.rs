use anyhow::Result;
use async_trait::async_trait;
pub mod degarashi;
pub mod drawing;
pub mod kanteishi_saikyo;
#[macro_use]
pub mod macros;
pub mod elder_elite;
pub mod fe_engage;
pub mod game_of_familia;
pub mod kaihuku_jutsushi;
pub mod kuga;
pub mod maou_rebellion;
pub mod matoseihei;
pub mod one_punch_man;
pub mod rezero_ch4;
pub mod rta_kaerenai;
pub mod satanophany;
pub mod sentai_taboo;
pub mod shield_yusha;
pub mod shujin_tensei;
pub mod tensei_coliseum;
pub mod toaru_anbu;
pub mod toaru_shinri;
pub mod yondome;

#[async_trait]
pub trait MangaCrawler: Send + Sync {
    async fn crawl_latest_episode(&self, url: &str) -> Result<String>;
}

pub trait MangaInfo {
    fn short_title(&self) -> &str;
    fn title(&self) -> &str;
    fn url(&self) -> &str;
}

pub trait Manga: MangaCrawler + MangaInfo {}

pub trait IntoManga {
    fn into(self) -> Box<dyn Manga>;
}

impl<T> IntoManga for T
where
    T: Manga + 'static,
{
    fn into(self) -> Box<dyn Manga> {
        // Box の中身としてヒープ領域に格納するので、格納する値が先に drop すると困る
        // 'static 制約が必要
        Box::new(self)
    }
}
