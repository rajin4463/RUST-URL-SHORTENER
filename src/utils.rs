use axum::http::StatusCode;
use metrics::counter;

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    tracing::error!("{}", err);

    // Assuming you want to increment a counter named "request_error" with a label "error"
    counter!("request_error", "error" => format!("{}!", err)).increment(1);

    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
