use gloo_net::http::Request;
use crate::domain::model::auth::{AuthTokens, LoginRequest};

pub async fn login(req: LoginRequest) -> Result<AuthTokens, String> {
    let resp = Request::post("/api/v1/auth/login")
        .json(&req)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json::<AuthTokens>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Login xatosi: {}", resp.status()))
    }
}
