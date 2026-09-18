use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

pub async fn login(Path(provider): Path<String>) -> impl IntoResponse {
    match provider.as_str() {
        "google" => {
            Redirect::temporary("https://accounts.google.com/o/oauth2/v2/auth").into_response()
        }
        "github" => {
            Redirect::temporary("https://github.com/login/oauth/authorize").into_response()
        }
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn callback(Path(provider): Path<String>) -> impl IntoResponse {
    match provider.as_str() {
        "google" | "github" => StatusCode::OK.into_response(),
        _ => StatusCode::NOT_FOUND.into_response(),
    }
}
