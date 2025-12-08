use anyhow::Result;
use url::Url;

use crate::manga::portal::{
    manga_url::{MangaUrl, SameUrl, SplitUrl},
    portal_kind::PortalKind,
};

pub mod manga_url;
pub mod portal_kind;

#[derive(Debug, Clone)]
pub enum MangaPortal {
    WebAce(SplitUrl),
    KimiComi(SplitUrl),
    KadoComi(SameUrl),
    Tonarino(SplitUrl),
    HerosWeb(SplitUrl),
    JumpPlus(SplitUrl),
    YoungMagazine(SameUrl),
    ComicDays(SplitUrl),
    ComicFuz(SameUrl),
    ComicZenon(SplitUrl),
}

impl MangaPortal {
    pub fn new(kind: PortalKind, crawl: Url, public: Url) -> Result<Self> {
        let v = match kind {
            PortalKind::WebAce => MangaPortal::WebAce(SplitUrl::new(crawl, public)?),
            PortalKind::KimiComi => MangaPortal::KimiComi(SplitUrl::new(crawl, public)?),
            PortalKind::KadoComi => {
                if crawl != public {
                    return Err(anyhow::anyhow!(
                        "For KadoComi, crawl URL and public URL must be the same: {}, {}",
                        crawl,
                        public
                    ));
                }
                MangaPortal::KadoComi(SameUrl::new(crawl))
            }
            PortalKind::TonarinoYJ => MangaPortal::Tonarino(SplitUrl::new(crawl, public)?),
            PortalKind::HerosWeb => MangaPortal::HerosWeb(SplitUrl::new(crawl, public)?),
            PortalKind::JumpPlus => MangaPortal::JumpPlus(SplitUrl::new(crawl, public)?),
            PortalKind::YoungMagazine => {
                if crawl != public {
                    return Err(anyhow::anyhow!(
                        "For YoungMagazine, crawl URL and public URL must be the same: {}, {}",
                        crawl,
                        public
                    ));
                }
                MangaPortal::YoungMagazine(SameUrl::new(crawl))
            }
            PortalKind::ComicDays => MangaPortal::ComicDays(SplitUrl::new(crawl, public).unwrap()),
            PortalKind::ComicFuz => {
                if crawl != public {
                    return Err(anyhow::anyhow!(
                        "For ComicFuz, crawl URL and public URL must be the same: {}, {}",
                        crawl,
                        public
                    ));
                }
                MangaPortal::ComicFuz(SameUrl::new(crawl))
            }
            PortalKind::ComicZenon => {
                MangaPortal::ComicZenon(SplitUrl::new(crawl, public).unwrap())
            }
        };
        Ok(v)
    }
    pub fn kind(&self) -> PortalKind {
        match self {
            MangaPortal::WebAce(_) => PortalKind::WebAce,
            MangaPortal::KimiComi(_) => PortalKind::KimiComi,
            MangaPortal::KadoComi(_) => PortalKind::KadoComi,
            MangaPortal::Tonarino(_) => PortalKind::TonarinoYJ,
            MangaPortal::HerosWeb(_) => PortalKind::HerosWeb,
            MangaPortal::JumpPlus(_) => PortalKind::JumpPlus,
            MangaPortal::YoungMagazine(_) => PortalKind::YoungMagazine,
            MangaPortal::ComicDays(_) => PortalKind::ComicDays,
            MangaPortal::ComicFuz(_) => PortalKind::ComicFuz,
            MangaPortal::ComicZenon(_) => PortalKind::ComicZenon,
        }
    }
}

impl MangaUrl for MangaPortal {
    fn get_crawl_url(&self) -> &Url {
        match self {
            MangaPortal::WebAce(url)
            | MangaPortal::KimiComi(url)
            | MangaPortal::Tonarino(url)
            | MangaPortal::HerosWeb(url)
            | MangaPortal::JumpPlus(url)
            | MangaPortal::ComicDays(url)
            | MangaPortal::ComicZenon(url) => url.get_crawl_url(),
            MangaPortal::KadoComi(url)
            | MangaPortal::YoungMagazine(url)
            | MangaPortal::ComicFuz(url) => url.get_crawl_url(),
        }
    }

    fn get_public_url(&self) -> &Url {
        match self {
            MangaPortal::WebAce(url)
            | MangaPortal::KimiComi(url)
            | MangaPortal::Tonarino(url)
            | MangaPortal::HerosWeb(url)
            | MangaPortal::JumpPlus(url)
            | MangaPortal::ComicDays(url)
            | MangaPortal::ComicZenon(url) => url.get_public_url(),
            MangaPortal::KadoComi(url)
            | MangaPortal::YoungMagazine(url)
            | MangaPortal::ComicFuz(url) => url.get_public_url(),
        }
    }
}
