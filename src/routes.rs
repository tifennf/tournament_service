use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde_json::Value;

use crate::{
    core::{ApiResponse, SharedState, POOL_SIZE},
    middlewares::OpenCheckLayer,
    ressources::{Player, PlayerList, State, Tournament},
    utils,
};

pub async fn not_found() -> ApiResponse<&'static str> {
    ApiResponse::new(StatusCode::NOT_FOUND, "Not found")
}

pub fn root() -> Router {
    async fn handler() -> &'static str {
        "API de tournois TFT pour la structure Xpako\n\nConcepteur: https://github.com/tifennf"
    }

    utils::route("/", get(handler))
}

pub fn info() -> Router {
    async fn handler(
        Extension(state): Extension<SharedState>,
    ) -> Result<ApiResponse<State>, ApiResponse<Value>> {
        let state = utils::resolve_state(state.lock())?;

        Ok(ApiResponse::new(StatusCode::OK.into(), state.clone()))
    }

    utils::route("/info", get(handler))
}
fn register_player() -> Router {
    async fn handler(
        Extension(state): Extension<SharedState>,
        Json(player): Json<Player>,
    ) -> Result<ApiResponse<Value>, ApiResponse<Value>> {
        let mut state = utils::resolve_state(state.lock())?;

        let player_list = &mut state.player_list;

        if let Some(player_list) = player_list {
            player_list.insert(player);
        }
        Ok(ApiResponse::new(StatusCode::OK, Value::Null))
    }

    utils::route("/player", post(handler))
}

fn draw_pools() -> Router {
    async fn handler(
        Extension(state): Extension<SharedState>,
    ) -> Result<ApiResponse<Tournament>, ApiResponse<Value>> {
        let mut state = utils::resolve_state(state.lock())?;

        let player_list = state.player_list.as_ref();

        if let Some(player_list) = player_list {
            let mut tournament = Tournament::new(player_list.max_amount.0 / POOL_SIZE);

            tournament.fill(player_list.list());

            state.tournament = Some(tournament.clone());

            Ok(ApiResponse::new(StatusCode::OK, tournament))
        } else {
            Err(ApiResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                Value::Null,
            ))
        }
    }

    utils::route("/player", get(handler))
}

fn start_tournament() -> Router {
    async fn handler(
        Extension(state): Extension<SharedState>,
        amount: Option<Path<usize>>,
    ) -> Result<ApiResponse<Value>, ApiResponse<Value>> {
        let mut state = utils::resolve_state(state.lock())?;

        let amount = amount.unwrap_or(Path(64));

        state.player_list = Some(PlayerList::new(amount.0));
        state.open = true;

        Ok(ApiResponse::new(StatusCode::OK, Value::Null))
    }

    utils::route("/", put(handler))
}
fn stop_tournament() -> Router {
    async fn handler(
        Extension(state): Extension<SharedState>,
    ) -> Result<ApiResponse<Value>, ApiResponse<Value>> {
        let mut state = utils::resolve_state(state.lock())?;

        *state = State::default();

        Ok(ApiResponse::new(StatusCode::OK, Value::Null))
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
