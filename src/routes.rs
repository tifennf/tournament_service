use std::sync::{Arc, Mutex};

use axum::{
    extract::Extension,
    response::IntoResponse,
    routing::{get, MethodRouter},
    Json, Router,
};

use crate::{
    ressources::{Player, State, Tournament},
    utils::{self, route},
};

pub fn root() -> Router {
    pub async fn handler() -> &'static str {
        "test ok"
    }

    route("/", get(handler))
}

pub fn register_player() -> Router {
    async fn handler(Json(payload): Json<Player>) -> impl IntoResponse {
        format!("{:#?}", payload)
    }
    utils::route("/player", get(handler))
}

pub fn print_tournament() -> Router {
    async fn handler(Extension(state): Extension<Arc<Mutex<State>>>) -> Json<Tournament> {
        let state = state.lock().unwrap();

        Json(state.tournament.clone())
    }

    utils::route("/tournament", get(handler))
}
