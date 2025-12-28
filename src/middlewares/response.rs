use axum::{Json, http::StatusCode};
use serde::Serialize;
use std::usize;

pub type AxumResponse<T> = (StatusCode, Json<JsonResponse<T>>);

#[derive(Debug, Serialize)]
pub struct JsonResponse<T: Serialize> {
    status: u16,
    data: Option<T>,
    message: String,
}

impl<T: Serialize> JsonResponse<T> {
    fn new(status: u16, data: Option<T>, message: String) -> Self {
        Self {
            status,
            data,
            message,
        }
    }

    pub fn send(status: StatusCode, data: Option<T>, message: Option<String>) -> AxumResponse<T> {
        let new_message = match message {
            Some(msg) => msg,
            None => match data {
                Some(_) => status.to_string(),
                None => "".to_string(),
            },
        };
        let response = Self::new(status.as_u16(), data, new_message);
        (status, Json(response))
    }
}
