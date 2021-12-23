use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{AddExtensionLayer, Router};
use tower_http::trace::TraceLayer;

use crate::{
    ressources::{State, Tournament},
    routes::{print_tournament, register_player, root},
};

pub async fn run(addr: &SocketAddr) {
    let state = Arc::new(Mutex::new(State {
        tournament: Tournament::new(),
    }));

    let app = Router::new()
        .merge(root())
        .merge(register_player())
        .merge(print_tournament())
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(state));

    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
