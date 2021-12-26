use std::{
    collections::HashSet,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{AddExtensionLayer, Router};
use tower_http::trace::TraceLayer;

use crate::{
    ressources::State,
    routes::{info, manage_tournament, root},
};

pub async fn run(addr: &SocketAddr) {
    let state = Arc::new(Mutex::new(State {
        tournament: None,
        player_list: HashSet::new(),
        open: false,
    }));

    let app = Router::new()
        .merge(manage_tournament())
        .merge(info())
        .merge(root())
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(state));

    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
