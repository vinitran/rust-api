mod handlers;
mod extractors;
mod error;

use handlers::{user, auth};
use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use extractors::state::AppState;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();

    shared::logging::set_up("http_server=debug");

    let state = {
        let url_db = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");
        AppState::new(&url_db).await.expect("Failed to create app state")
    };

    let auth_router = Router::new()
        .route("/auth/{id}", post(auth::login::handler));

    let user_router = Router::new()
        .route("/user/{id}", get(user::me::handler));

    let app = Router::new()
        .route("/", get(|| async { "🦀 hello kitty !" }))
        .merge(user_router)
        .merge(auth_router)
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::info!("🦀 server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}