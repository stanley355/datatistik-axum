use axum::{extract::Request, http::header::COOKIE, middleware::Next, response::Response};

use crate::middlewares::AxumResponse;

pub struct BetterAuth {}
impl BetterAuth {
    pub async fn middleware(req: Request, next: Next) -> Result<Response, AxumResponse<String>> {
        let header_cookie = req.headers().get(COOKIE);
        if let Some(cookie_header) = header_cookie.and_then(|h| h.to_str().ok()) {
            // 2. Parse the string (Cookies are separated by '; ')
            let session_cookie = cookie_header
                .split("; ")
                .find(|cookie| cookie.starts_with("session_id="));

            if let Some(cookie_kv) = session_cookie {
                let value = &cookie_kv["session_id=".len()..];
                println!("Found raw session cookie: {}", value);

                // Do your auth validation here...
            }
        }
        Ok(next.run(req).await)
    }
}
