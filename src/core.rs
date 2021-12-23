use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{AddExtensionLayer, Router};
use tower_http::trace::TraceLayer;

use crate::{
    ressources::State,
    routes::{register_player, root, tournament},
};

pub async fn run(addr: &SocketAddr) {
    let state = Arc::new(Mutex::new(State {
        tournament: None,
        player_list: Vec::new(),
    }));

    let app = Router::new()
        .merge(root())
        .merge(register_player())
        .merge(tournament())
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(state));

    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
