use tournament_service::handlers::{register_player, root};

use axum::routing::{get, post};
use axum::{AddExtensionLayer, Router};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tournament_service::ressources::Tournament;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "tournament_service=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let state = Arc::new(Mutex::new(Tournament::new()));

    let app = Router::new()
        .route("/", get(root))
        .route("/player", post(register_player))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("Listening on address: {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
