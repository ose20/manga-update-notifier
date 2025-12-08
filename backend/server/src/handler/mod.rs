use axum::http::StatusCode;

pub mod manga;

pub type MyRoughlyErrResponse = (StatusCode, String);

// どのエラーでも 500 にしてメッセージだけ返す
// Todo: あとでちゃんとエラーメッセージを丁寧にする
fn into_500<E: std::fmt::Display>(err: E) -> MyRoughlyErrResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
