use axum::{
    extract::Extension,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde_json::Value;

use crate::{
    core::{ApiResponse, SharedState, POOL_SIZE},
    middlewares::OpenCheckLayer,
    ressources::{InitTournament, InscriptionsState, Player, PlayerList, State, Tournament},
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

        let player_list = state.player_list.as_mut().ok_or(ApiResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            Value::Null,
        ))?;

        let status = if player_list.insert(player) {
            StatusCode::OK
        } else {
            StatusCode::FORBIDDEN
        };

        Ok(ApiResponse::new(status, Value::Null))
    }

    utils::route("/inscriptions", post(handler))
}

fn start_tournament() -> Router {
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

    utils::route("/", get(handler))
}

fn init_tournament() -> Router {
    async fn handler(
        Extension(state): Extension<SharedState>,
        Json(body): Json<InitTournament>,
    ) -> Result<ApiResponse<Value>, ApiResponse<Value>> {
        let mut state = utils::resolve_state(state.lock())?;

        let amount = body.max_player;

        state.player_list = Some(PlayerList::new(amount));

        Ok(ApiResponse::new(StatusCode::OK, Value::Null))
    }

    utils::route("/", post(handler))
}
fn reset_tournament() -> Router {
    async fn handler(
        Extension(state): Extension<SharedState>,
    ) -> Result<ApiResponse<Value>, ApiResponse<Value>> {
        let mut state = utils::resolve_state(state.lock())?;

        *state = State::default();

        Ok(ApiResponse::new(StatusCode::OK, Value::Null))
    }

    utils::route("/reset", get(handler))
}
fn open_inscriptions() -> Router {
    async fn handler(
        Extension(state): Extension<SharedState>,
        inscriptions: Option<Json<InscriptionsState>>,
    ) -> Result<ApiResponse<bool>, ApiResponse<Value>> {
        let mut state = utils::resolve_state(state.lock())?;

        match inscriptions {
            Some(Json(inscriptions)) => {
                let open = inscriptions.open;

                state.open = open;

                Ok(ApiResponse::new(StatusCode::OK, open))
            }
            None => Err(ApiResponse::new(
                StatusCode::BAD_REQUEST,
                "You need to precise if true or false".into(),
            )),
        }
    }

    utils::route("/inscriptions/status", post(handler))
}

pub fn manage_tournament() -> Router {
    let tournament_routes = Router::new()
        .merge(start_tournament())
        .merge(register_player())
        .layer(OpenCheckLayer)
        .merge(open_inscriptions())
        .merge(init_tournament())
        .merge(reset_tournament());

    Router::new().nest("/tournament", tournament_routes)
}
