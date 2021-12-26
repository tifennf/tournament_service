use std::sync::{Arc, Mutex};

use ressources::State;

pub mod core;
pub mod middlewares;
pub mod ressources;
pub mod routes;
pub mod utils;

pub const POOL_AMOUNT: usize = 8;
pub const POOL_MAX_SIZE: usize = 8;
pub const PLAYER_AMOUNT: usize = 64;

pub type SharedState = Arc<Mutex<State>>;
