use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PortalKind {
    WebAce,
    KimiComi,
    KadoComi,
    TonarinoYJ,
    HerosWeb,
    JumpPlus,
    YoungMagazine,
    ComicDays,
    ComicFuz,
}

// 考察:
// DBから取ってきた値を変換する時に使う処理であるこれを domain 層に書いていいのか？
// しかしこれは、PortalKind の生成ルールであると考えると domain 層に置くのが自然にも思える
// この型の値を生成できる条件はこっちに書いておきたい気がする
impl FromStr for PortalKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "WebAce" => Ok(PortalKind::WebAce),
            "KimiComi" => Ok(PortalKind::KimiComi),
            "KadoComi" => Ok(PortalKind::KadoComi),
            "TonarinoYJ" => Ok(PortalKind::TonarinoYJ),
            "HerosWeb" => Ok(PortalKind::HerosWeb),
            "JumpPlus" => Ok(PortalKind::JumpPlus),
            "YoungMagazine" => Ok(PortalKind::YoungMagazine),
            "ComicDays" => Ok(PortalKind::ComicDays),
            "ComicFuz" => Ok(PortalKind::ComicFuz),
            _ => Err(anyhow::anyhow!("Unknown portal kind: {}", s)),
        }
    }
}
