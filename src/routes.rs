use axum::{
    extract::Extension,
    handler::Handler,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use tracing::debug;

use crate::{
    middlewares::{OpenCheckLayer, PlayerCheckLayer},
    ressources::{Player, State, Tournament},
    utils, SharedState,
};

pub fn root() -> Router {
    pub async fn handler() -> &'static str {
        "API de tournois TFT pour la structure Xpako\n\nConcepteur: https://github.com/tifennf"
    }

    utils::route("/", get(handler))
}

pub fn register_player() -> Router {
    async fn handler(
        Extension(state): Extension<SharedState>,
        Json(player): Json<Player>,
    ) -> impl IntoResponse {
        let mut state = state.lock().unwrap();

        let player_list = &mut state.player_list;

        player_list.insert(player);

        StatusCode::OK
    }

    let handler = handler.layer(PlayerCheckLayer);

    utils::route("/player", post(handler))
}

fn draw_pools() -> Router {
    async fn handler(Extension(state): Extension<SharedState>) -> impl IntoResponse {
        let mut state = state.lock().unwrap();

        let mut tournament = Tournament::new();

        tournament.fill(state.player_list.clone());

        state.tournament = Some(tournament.clone());

        Json(tournament)
    }

    utils::route("/player", get(handler))
}

fn print_tournament() -> Router {
    async fn handler(Extension(state): Extension<SharedState>) -> Json<Option<Tournament>> {
        let state = state.lock().unwrap();

        Json(state.tournament.clone())
    }

    utils::route("/", get(handler))
}
pub fn start_tournament() -> Router {
    async fn handler(Extension(state): Extension<SharedState>) -> impl IntoResponse {
        let mut state = state.lock().unwrap();

        state.open = true;

        debug!("{:?}", state);

        StatusCode::IM_A_TEAPOT
    }

    utils::route("/", put(handler))
}
fn stop_tournament() -> Router {
    async fn handler(Extension(state): Extension<SharedState>) -> impl IntoResponse {
        let mut state = state.lock().unwrap();

        *state = State::default();

        StatusCode::OK
    }

    utils::route("/", delete(handler))
}

pub fn manage_tournament() -> Router {
    let tournament_routes = Router::new()
        .merge(draw_pools())
        .merge(register_player())
        .layer(OpenCheckLayer)
        .merge(print_tournament())
        .merge(start_tournament())
        .merge(stop_tournament());

    Router::new().nest("/tournament", tournament_routes)
}
