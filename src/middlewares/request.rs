use axum::{extract::Request, http::header::COOKIE, middleware::Next, response::Response};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    envs::Envs,
    middlewares::{AxumResponse, JsonResponse},
};

pub struct BetterAuth {}

#[derive(Debug, Deserialize)]
struct BetterAuthSession {
    #[allow(dead_code)]
    id: String,
}

#[derive(Debug, Deserialize, Clone)]
struct BetterAuthUser {
    #[allow(dead_code)]
    id: String,
    role: String,
}

#[derive(Debug, Deserialize)]
struct BetterAuthGetSession {
    #[allow(dead_code)]
    session: BetterAuthSession,
    user: BetterAuthUser,
}

impl BetterAuth {
    async fn check_session(cookie: &str) -> Result<reqwest::Response, reqwest::Error> {
        let better_auth_url = Envs::better_auth_url();
        let url = format!("{}{}", better_auth_url, "/api/auth/get-session");

        reqwest::Client::new()
            .get(url)
            .header(COOKIE, cookie)
            .send()
            .await
    }

    pub async fn admin_middleware(
        req: Request,
        next: Next,
    ) -> Result<Response, AxumResponse<String>> {
        let header_cookie = req.headers().get(COOKIE);
        if let Some(cookie) = header_cookie.and_then(|h| h.to_str().ok()) {
            let session = match Self::check_session(cookie).await {
                Ok(s) => s,
                Err(err) => {
                    let status = match err.status() {
                        Some(stat) => stat,
                        None => StatusCode::INTERNAL_SERVER_ERROR,
                    };
                    let response = JsonResponse::send(status, None, Some(err.to_string()));
                    return Err(response);
                }
            };

            if let Ok(session_data) = session.json::<BetterAuthGetSession>().await {
                if session_data.user.role == "admin" {
                    return Ok(next.run(req).await);
                }
            }
        }
        let response = JsonResponse::send(
            StatusCode::UNAUTHORIZED,
            None,
            Some("Unauthorized".to_string()),
        );
        Err(response)
    }
}
