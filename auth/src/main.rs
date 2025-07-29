use axum::Router;
mod controllers;
mod routes;
mod repositories;
mod services;
mod helpers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().nest("/auth", routes::auth::auth_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    tracing::info!("🚀 Serveur en ligne sur http://localhost:3000");
}
