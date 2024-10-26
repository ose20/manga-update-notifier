use anyhow::Result;
use futures::future;
use shared::config;

use crate::{model::MangaExistenceList, open};

#[derive(clap::Args, Debug)]
pub struct Args {}

pub async fn run(_args: Args) -> Result<()> {
    let app_config = config::AppConfig::new()?;
    let app_registry = registry::AppRegistry::new(&app_config);
    let file_path = "./batch/config/existence.yaml";
    let reader = open(file_path)?;
    let manga_list: MangaExistenceList = serde_yaml::from_reader(reader)?;

    let futures_list = manga_list.mangas.into_iter().map(|manga| async {
        app_registry
            .manga_repository()
            .update_persistence(manga.into())
            .await
    });

    let results = future::join_all(futures_list).await;
    let affected_rows: u64 = results
        .into_iter()
        .map(|result| match result {
            Ok(count) => count,
            Err(e) => {
                eprintln!("err: 漫画のデータ更新でエラーが発生\n{:#?}", e);
                0
            }
        })
        .sum();

    println!("affected rows: {}", affected_rows);
    Ok(())
}
