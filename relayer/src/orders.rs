use axum::{Router, http::StatusCode, routing::get};

pub fn router() -> Router {
    Router::new()
        .route("/", get(async || StatusCode::OK))
        .route("/v1.0/order/active", get(async || StatusCode::OK))
}
