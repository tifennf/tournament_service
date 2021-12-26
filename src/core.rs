use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{AddExtensionLayer, Router};
use tower_http::trace::TraceLayer;

use crate::{
    middlewares::{OpenCheckLayer, RequestGuard},
    ressources::State,
    routes::{manage_tournament, register_player, root},
};

pub async fn run(addr: &SocketAddr) {
    let state = Arc::new(Mutex::new(State {
        tournament: None,
        player_list: Vec::new(),
        open: true,
    }));

    let app = Router::new()
        .merge(manage_tournament())
        .merge(root())
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(state));

    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
