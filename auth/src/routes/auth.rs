use axum::{routing::post, Router};
use crate::controllers;

pub fn auth_routes() -> Router {
    Router::new()
        .route("/login", post(controllers::auth::login))
        .route("/register", post(controllers::auth::register))
}
