use std::sync::{Arc, Mutex};

use axum::{
    extract::Extension,
    response::IntoResponse,
    routing::{delete, get, post},
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

fn print_tournament() -> Router {
    async fn handler(Extension(state): Extension<Arc<Mutex<State>>>) -> Json<Option<Tournament>> {
        let state = state.lock().unwrap();

        Json(state.tournament.clone())
    }

    utils::route("/tournament", get(handler))
}

fn start_tournament() -> Router {
    async fn handler(Extension(state): Extension<Arc<Mutex<State>>>) {
        let mut state = state.lock().unwrap();

        state.tournament = Some(Tournament::new());
    }

    utils::route("/tournament", post(handler))
}
fn stop_tournament() -> Router {
    async fn handler(Extension(state): Extension<Arc<Mutex<State>>>) {
        let mut state = state.lock().unwrap();

        state.tournament = None;
    }

    utils::route("/tournament", delete(handler))
}

pub fn tournament() -> Router {
    let svc = Router::new()
        .merge(print_tournament())
        .merge(start_tournament())
        .merge(stop_tournament());

    Router::new().nest("/", svc)
}
