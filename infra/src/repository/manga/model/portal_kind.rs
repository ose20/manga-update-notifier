use domain::manga::PortalKind;

pub struct PortalKindValue(String);

impl PortalKindValue {
    pub fn inner(self) -> String {
        self.0
    }
}

// enum として表現された domain モデルをどうやって永続化するかという話なので
// これは infra 層に置くのが自然に思える
impl From<PortalKind> for PortalKindValue {
    fn from(kind: PortalKind) -> Self {
        let s = match kind {
            PortalKind::WebAce => "WebAce",
            PortalKind::KimiComi => "KimiComi",
            PortalKind::KadoComi => "KadoComi",
            PortalKind::TonarinoYJ => "TonarinoYJ",
            PortalKind::HerosWeb => "HerosWeb",
            PortalKind::JumpPlus => "JumpPlus",
            PortalKind::YoungMagazine => "YoungMagazine",
            PortalKind::ComicDays => "ComicDays",
            PortalKind::ComicFuz => "ComicFuz",
            PortalKind::ComicZenon => "ComicZenon",
        };
        PortalKindValue(s.to_string())
    }
}
