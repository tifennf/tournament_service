use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use tracing::debug;

use crate::{
    middlewares::OpenCheckLayer,
    ressources::{Player, PlayerList, State, Tournament},
    utils, SharedState, POOL_SIZE,
};

pub fn root() -> Router {
    pub async fn handler() -> &'static str {
        "API de tournois TFT pour la structure Xpako\n\nConcepteur: https://github.com/tifennf"
    }

    utils::route("/", get(handler))
}

pub fn info() -> Router {
    async fn handler(Extension(state): Extension<SharedState>) -> Json<State> {
        let state = state.lock().unwrap();

        Json(state.clone())
    }

    utils::route("/info", get(handler))
}
pub fn register_player() -> Router {
    async fn handler(
        Extension(state): Extension<SharedState>,
        Json(player): Json<Player>,
    ) -> impl IntoResponse {
        let mut state = state.lock().unwrap();

        let player_list = &mut state.player_list;

        if let Some(player_list) = player_list {
            player_list.insert(player);
        }

        StatusCode::OK
    }

    utils::route("/player", post(handler))
}

fn draw_pools() -> Router {
    async fn handler(Extension(state): Extension<SharedState>) -> impl IntoResponse {
        let mut state = state.lock().unwrap();

        let player_list = state.player_list.as_ref().unwrap();
        let mut tournament = Tournament::new(player_list.max_amount.0 / POOL_SIZE);

        tournament.fill(player_list.list());

        state.tournament = Some(tournament.clone());

        Json(tournament)
    }

    utils::route("/player", get(handler))
}

pub fn start_tournament() -> Router {
    async fn handler(Extension(state): Extension<SharedState>) -> impl IntoResponse {
        let mut state = state.lock().unwrap();

        state.player_list = Some(PlayerList::new(64));
        state.open = true;

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
        .merge(start_tournament())
        .merge(stop_tournament());

    Router::new().nest("/tournament", tournament_routes)
}
