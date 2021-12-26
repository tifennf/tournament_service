use std::sync::{LockResult, MutexGuard};

use axum::{http::StatusCode, routing::MethodRouter, Router};
use rand::{prelude::SliceRandom, thread_rng};
use serde_json::Value;

use crate::{
    core::{ApiResponse, POOL_SIZE},
    ressources::{Player, Pool, State},
};

pub fn make_pools(amount: usize) -> Vec<Pool> {
    (0..amount)
        .into_iter()
        .map(|n| Pool::new(n, POOL_SIZE))
        .collect()
}

pub fn route(path: &str, method_router: MethodRouter) -> Router {
    Router::new().route(path, method_router)
}

pub fn shuffle_players(mut list: Vec<Player>) -> Vec<Player> {
    let mut rng = thread_rng();

    list.shuffle(&mut rng);

    list
}

pub fn generate_players(amount: usize) -> Vec<Player> {
    let mut list = Vec::new();

    let mut i = 0;

    while i < amount {
        let player = Player {
            name: i.to_string(),
        };

        list.push(player);

        i += 1;
    }

    list
}

pub fn resolve_state(
    state: LockResult<MutexGuard<'_, State>>,
) -> Result<MutexGuard<State>, ApiResponse<Value>> {
    state.map_err(|err| {
        tracing::error!("Error on lock: {}", err);
        ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, Value::Null)
    })
}
