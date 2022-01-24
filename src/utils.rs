use std::{
    fs,
    io::{BufReader, BufWriter, Write},
    sync::{LockResult, MutexGuard},
};

use axum::{http::StatusCode, routing::MethodRouter, Router};
use rand::{prelude::SliceRandom, thread_rng};
use serde_json::Value;

use crate::{
    core::{ApiResponse, State, POOL_SIZE},
    ressources::{DiscordName, PlayerList, PlayerVerified, Pool},
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

pub fn shuffle_players(mut list: Vec<PlayerVerified>) -> Vec<PlayerVerified> {
    let mut rng = thread_rng();

    list.shuffle(&mut rng);

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

pub fn save_plist(list: &PlayerList) -> Result<(), String> {
    let file = fs::File::create("./players.json").map_err(|err| err.to_string())?;

    let mut writer = BufWriter::new(&file);
    serde_json::to_writer_pretty(&mut writer, list).map_err(|err| err.to_string())?;

    writer.flush().map_err(|err| err.to_string())?;

    Ok(())
}
pub fn get_plist() -> Result<PlayerList, String> {
    let file = fs::File::open("./players.json").map_err(|err| err.to_string())?;

    let reader = BufReader::new(&file);

    serde_json::from_reader(reader).map_err(|err| err.to_string())
}

// pub fn fake_player() -> Result<PlayerVerified, ApiResponse<Value>> {
//     let discord_name = DiscordName::new("FAUX JOUEUR".to_string(), 0)
//         .map_err(|_| ApiResponse::new(StatusCode::INTERNAL_SERVER_ERROR, Value::Null))?;

//     let p = PlayerVerified {
//         league_name: "FAUX JOUEUR POUR FIX LE BOT".to_string(),
//         discord_name,
//         discord_id: "2n+1".to_string(),
//     };

//     Ok(p)
// }
