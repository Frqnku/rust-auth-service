use axum::{Extension, Router};
mod controllers;
mod helpers;
mod repositories;
mod routes;
mod services;
use shared::database::init_db;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db_pool    = init_db().await;

    let app = Router::new()
        .nest("/auth", routes::auth::auth_routes())
        .layer(Extension(db_pool.clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    tracing::info!("🚀 Serveur en ligne sur http://localhost:3000");
}
