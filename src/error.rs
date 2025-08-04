use axum::{http::StatusCode, response::IntoResponse};

pub struct MyStatusCode(StatusCode);

impl From<minijinja::Error> for MyStatusCode {
    fn from(error: minijinja::Error) -> Self {
        tracing::error!(error = ?error, "Minijinja error");
        MyStatusCode(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl IntoResponse for MyStatusCode {
    fn into_response(self) -> axum::response::Response {
        self.0.into_response()
    }
}
