use anyhow::Result;
use domain::manga::MangaEpisode;
use shared::config::WebdriverConfig;
use std::sync::Mutex;
use std::{collections::VecDeque, sync::Arc};
use thirtyfour::prelude::*;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use crate::fetcher::portal_kind::EpCrawler;

pub struct DriverPool {
    // まさにこいつが管理したい資源
    drivers: Mutex<VecDeque<WebDriver>>,
    // aquire で借用の permit ではなく OwnedPermit を獲得するために Arc
    // ただの permit だと Send じゃなかったり、ライフタイム管理が微妙になる
    // 貸出権利証の管理(同時にn枚まで)
    sem: Arc<Semaphore>,
}

pub struct Lease<'a> {
    // こいつが借りている WebDriver の持ち主
    pool: &'a DriverPool,
    // 借りている WebDriver
    driver: Option<WebDriver>,
    // driver とライフタイムを合わせるためにもつ
    // OwnedSemaphorePermit は Drop で自動解放される
    // WebDriver を借りている間だけ保持される貸出権利証
    _permit: OwnedSemaphorePermit,
}

impl Lease<'_> {
    pub fn driver(&self) -> &WebDriver {
        self.driver.as_ref().unwrap()
    }
}

impl Drop for Lease<'_> {
    // Lease がスコープを抜けるときに自動で返却する(RAII)
    fn drop(&mut self) {
        if let Some(drv) = self.driver.take() {
            // 同期 Mutex なので await 不要 → Drop 中でも安全に返せる
            let mut q = self.pool.drivers.lock().unwrap();
            q.push_back(drv);
        }
        // _permit は自動で解放
    }
}

impl DriverPool {
    pub async fn new(config: WebdriverConfig) -> WebDriverResult<Self> {
        let mut q = VecDeque::with_capacity(config.endpoints.len());
        for ep in config.endpoints.iter() {
            q.push_back(WebDriver::new(ep.to_string(), config.chrome_caps.clone()).await?);
        }
        Ok(Self {
            drivers: Mutex::new(q),
            sem: Arc::new(Semaphore::new(config.endpoints.len())),
        })
    }

    pub async fn acquire(&self) -> Lease<'_> {
        // acquire_owned するために Arc が必要
        let permit = self.sem.clone().acquire_owned().await.unwrap();
        let drv = {
            let mut q = self.drivers.lock().unwrap();
            q.pop_front().expect("logic error: semaphore > pool")
        };
        Lease {
            pool: self,
            driver: Some(drv),
            _permit: permit,
        }
    }

    // 使い方：await で借りて、スコープを抜けると自動返却
    pub async fn with_driver<C>(&self, crawler: C) -> Result<MangaEpisode>
    where
        C: EpCrawler,
    {
        let lease = self.acquire().await;
        crawler.crawl(lease.driver()).await
    }
}
