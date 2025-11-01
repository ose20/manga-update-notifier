use anyhow::anyhow;
use anyhow::Result;
use checker::degarashi::Degarashi;
use checker::drawing::Drawing;
use checker::elder_elite::ElderElite;
use checker::fe_engage::FeEngage;
use checker::game_of_familia::GameOfFamilia;
use checker::kaihuku_jutsushi::KaihukuJutsushi;
use checker::kanteishi_saikyo::KanteishiSaikyo;
use checker::kuga::Kuga;
use checker::maou_rebellion::MaouRebellion;
use checker::matoseihei::Matoseihei;
use checker::one_punch_man::OnePunchMan;
use checker::rezero_ch4::RezeroCh4;
use checker::rta_kaerenai::RtaKaerenai;
use checker::satanophany::Satanophany;
use checker::sentai_taboo::SentaiTaboo;
use checker::shield_yusha::ShieldYusha;
use checker::shujin_tensei::ShujinTensei;
use checker::tensei_coliseum::TenseiColiseum;
use checker::toaru_anbu::ToaruAnbu;
use checker::toaru_shinri::ToaruShinri;
use checker::yondome::Yondome;
use checker::IntoManga;
use checker::Manga;
use registry::AppRegistry;

pub mod aux;
pub mod checker;
pub mod chromedriver;

/// Result<Option<String>> は成功した場合は更新があった場合は Some((title, episode, url)), 更新がなかった場合は None
pub async fn check_update(
    app_registry: &AppRegistry,
) -> Vec<Result<Option<(String, String, String)>>> {
    // crawler を impl した各漫画 struct をひとつずつ作っていって Vec<impl crawler> をつくる
    let manga_list = register_manga_crawlers(app_registry).await;

    let mut results = vec![];

    for manga in manga_list {
        match manga {
            Ok(manga) => results.push(check_update_one(manga, app_registry).await),
            Err(e) => results.push(Err(e)),
        }
    }

    results
}

// 新しい漫画を追加したときはここで DI する
async fn register_manga_crawlers(app_registry: &AppRegistry) -> Vec<Result<Box<dyn Manga>>> {
    // dyn Manga の依存注入
    // 一つ一つ追加する
    let mut result = vec![];
    let degarashi = Degarashi::try_init(app_registry).await;
    // degarashiって'staticじゃないよね？なんで IntoManga::into 適用できる？
    // 'static な型とは参照を含まないか、'&static な参照しか含まない型のことなので degarashi は 'static
    result.push(degarashi.map(IntoManga::into));

    let kanteishi_saikyo = KanteishiSaikyo::try_init(app_registry).await;
    result.push(kanteishi_saikyo.map(IntoManga::into));

    let drawing = Drawing::try_init(app_registry).await;
    result.push(drawing.map(IntoManga::into));

    let yondome = Yondome::try_init(app_registry).await;
    result.push(yondome.map(IntoManga::into));

    let rta_kaerenai = RtaKaerenai::try_init(app_registry).await;
    result.push(rta_kaerenai.map(IntoManga::into));

    let one_punch_man = OnePunchMan::try_init(app_registry).await;
    result.push(one_punch_man.map(IntoManga::into));

    let kuga = Kuga::try_init(app_registry).await;
    result.push(kuga.map(IntoManga::into));

    let fe_engage = FeEngage::try_init(app_registry).await;
    result.push(fe_engage.map(IntoManga::into));

    let matoseihei = Matoseihei::try_init(app_registry).await;
    result.push(matoseihei.map(IntoManga::into));

    let toaru_anbu = ToaruAnbu::try_init(app_registry).await;
    result.push(toaru_anbu.map(IntoManga::into));

    let tensei_coliseum = TenseiColiseum::try_init(app_registry).await;
    result.push(tensei_coliseum.map(IntoManga::into));

    let elder_elite = ElderElite::try_init(app_registry).await;
    result.push(elder_elite.map(IntoManga::into));

    let maou_rebellion = MaouRebellion::try_init(app_registry).await;
    result.push(maou_rebellion.map(IntoManga::into));

    let game_of_familia = GameOfFamilia::try_init(app_registry).await;
    result.push(game_of_familia.map(IntoManga::into));

    let kaihuku_jutsushi = KaihukuJutsushi::try_init(app_registry).await;
    result.push(kaihuku_jutsushi.map(IntoManga::into));

    let satanophany = Satanophany::try_init(app_registry).await;
    result.push(satanophany.map(IntoManga::into));

    let sentai_taboo = SentaiTaboo::try_init(app_registry).await;
    result.push(sentai_taboo.map(IntoManga::into));

    let shujin_tensei = ShujinTensei::try_init(app_registry).await;
    result.push(shujin_tensei.map(IntoManga::into));

    let toaru_shinri = ToaruShinri::try_init(app_registry).await;
    result.push(toaru_shinri.map(IntoManga::into));

    let shield_yusha = ShieldYusha::try_init(app_registry).await;
    result.push(shield_yusha.map(IntoManga::into));

    let rezero_ch4 = RezeroCh4::try_init(app_registry).await;
    result.push(rezero_ch4.map(IntoManga::into));

    result
}

/// Result<Option<title, episode, url>> を返す
async fn check_update_one(
    manga: Box<dyn Manga>,
    app_registry: &AppRegistry,
) -> Result<Option<(String, String, String)>> {
    let manga_data = app_registry
        .manga_repository()
        .find_by_short_title(manga.short_title())
        .await?
        .ok_or(anyhow!(
            "漫画がDBに見つかりませんでした: title:{}",
            manga.title()
        ))?;
    let old_episode = manga_data.episode;
    let latest_episode = manga.crawl_latest_episode(manga.url()).await.map_err(|e| {
        anyhow!(
            "err: 最新話のクロール\ntitle: {}\ncontent: {:#?}",
            manga.title(),
            e
        )
    })?;

    if old_episode == latest_episode {
        Ok(None)
    } else {
        let update_result = app_registry
            .manga_repository()
            .update_episode_by_short_title(manga.short_title(), &latest_episode)
            .await;
        match update_result {
            Ok(_) => Ok(Some((
                manga.title().into(),
                latest_episode,
                manga.url().into(),
            ))),
            Err(e) => Err(anyhow!(
                "最新話の更新がありましたが、DBの更新に失敗しました\ntitle: {}\n{}",
                manga.title(),
                e
            )),
        }
    }
}
