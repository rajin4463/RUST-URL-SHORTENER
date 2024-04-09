use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "Service is Healthy")
}