use crate::manga::MangaTitle;

// ただのエラーロギングじゃなくて、イベントとして扱うための構造体
// ちょっと特殊だが、失敗したら discord などに通知してくれることでデバッグの契機としたいため
// ユーザーへのエラー通知が重要な意味を持ち、そうなるとドメインイベントとした方がいいと判断した
// 軽量のアプリで監視とかは特に入れないためこういう扱いにする
#[derive(Debug)]
pub struct DetectFetchError {
    pub title: MangaTitle,
    pub url: url::Url,
    pub error_message: String,
}

impl DetectFetchError {
    pub fn new(title: MangaTitle, url: url::Url, error_message: String) -> Self {
        Self {
            title,
            url,
            error_message,
        }
    }
}

impl std::fmt::Display for DetectFetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to fetch latest episode for manga '{}' ({}): {}",
            self.title.inner_ref(),
            self.url,
            self.error_message
        )
    }
}
