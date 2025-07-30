use crate::controllers::auth;
use axum::{routing::post, Router};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/login", post(auth::login))
        .route("/register", post(auth::register))
}
