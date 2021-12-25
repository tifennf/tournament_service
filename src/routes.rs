use std::sync::{Arc, Mutex};

use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};

use crate::{
    ressources::{Player, State, Tournament},
    utils, PLAYER_AMOUNT, POOL_AMOUNT, POOL_MAX_SIZE,
};

pub fn root() -> Router {
    pub async fn handler() -> &'static str {
        "test ok"
    }

    utils::route("/", get(handler))
}

pub fn register_player() -> Router {
    async fn handler(
        Extension(state): Extension<Arc<Mutex<State>>>,
        Json(player): Json<Player>,
    ) -> impl IntoResponse {
        let mut state = state.lock().unwrap();

        let player_list = &mut state.player_list;

        if player_list.len() < PLAYER_AMOUNT {
            player_list.push(player);

            StatusCode::OK
        } else {
            StatusCode::FORBIDDEN
        }
    }
    utils::route("/player", post(handler))
}

fn print_tournament() -> Router {
    async fn handler(Extension(state): Extension<Arc<Mutex<State>>>) -> Json<Option<Tournament>> {
        let state = state.lock().unwrap();

        Json(state.tournament.clone())
    }

    utils::route("/", get(handler))
}

fn start_tournament() -> Router {
    async fn handler(Extension(state): Extension<Arc<Mutex<State>>>) -> impl IntoResponse {
        let mut state = state.lock().unwrap();

        state.open = true;
    }

    utils::route("/", put(handler))
}
fn draw_pools() -> Router {
    async fn handler(Extension(state): Extension<Arc<Mutex<State>>>) -> impl IntoResponse {
        let mut state = state.lock().unwrap();

        if !state.open {
            StatusCode::FORBIDDEN
        } else {
            let mut tournament = Tournament::new();

            tournament.fill(state.player_list.clone());

            state.tournament = Some(tournament);

            StatusCode::OK
        }
    }

    utils::route("/", put(handler))
}
fn stop_tournament() -> Router {
    async fn handler(Extension(state): Extension<Arc<Mutex<State>>>) {
        let mut state = state.lock().unwrap();

        state.tournament = None;
    }

    utils::route("/", delete(handler))
}

pub fn tournament() -> Router {
    let tournament_routes = Router::new()
        .merge(print_tournament())
        .merge(start_tournament())
        .merge(stop_tournament());

    Router::new().nest("/tournament", tournament_routes)
}
