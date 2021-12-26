use std::sync::{Arc, Mutex};

use ressources::State;

pub mod core;
pub mod middlewares;
pub mod ressources;
pub mod routes;
pub mod utils;

pub const POOL_AMOUNT: [usize; 4] = [1, 2, 4, 8];
pub const PLAYER_AMOUNT: [usize; 4] = [8, 16, 32, 64];
pub const POOL_SIZE: usize = 8;

pub type SharedState = Arc<Mutex<State>>;
