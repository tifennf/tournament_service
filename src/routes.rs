use std::sync::{Arc, Mutex};

use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};

use crate::{
    ressources::{Player, State, Tournament},
    utils,
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
        let player_list = &mut state.lock().unwrap().player_list;

        if player_list.len() < 4 {
            player_list.push(player);
            println!("{:#?}", player_list);

            StatusCode::OK
        } else {
            println!("{:#?}", player_list);

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
    async fn handler(Extension(state): Extension<Arc<Mutex<State>>>) {
        let mut state = state.lock().unwrap();

        let mut tournament = Tournament::new();

        tournament.fill(utils::generate_players(64));

        state.tournament = Some(tournament);
    }

    utils::route("/", post(handler))
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
