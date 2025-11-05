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
