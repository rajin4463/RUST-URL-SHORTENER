use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sqlx::PgPool;
use crate::utils::internal_error;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub id: String,
    pub target_url: String,
}

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "Service is Healthy")
}

pub async fn redirect(
    State(pool): State<PgPool>,
    Path(request_link): Path<String>,
) -> Result<Response, (StatusCode, String)> {
    let link = sqlx::query_as!(
        Link,
        "select id, target_url from links where id = $1",
        request_link
    )
    .fetch_optional(&pool)
    .await
    .map_err(internal_error)?
    .ok_or_else(|| "Not Found".to_string())
    .map_err(|err| (StatusCode::NOT_FOUND, err))?;

    tracing::debug!(
        "Redirecting link id {} to {}",
        request_link,
        link.target_url
    );

    Ok(Response::builder()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .header("Location", link.target_url)
        .body(Body::empty())
        .expect("This response should always be constructable"))
}