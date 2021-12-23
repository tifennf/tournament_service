use serde::{Deserialize, Serialize};

use crate::utils;

const POOL_AMOUNT: u8 = 8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pool {
    player_list: Vec<Player>,
    id: u8,
    full: bool,
}

impl Pool {
    pub fn new(id: u8) -> Pool {
        Pool {
            player_list: Vec::new(),
            id,
            full: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tournament {
    pool_list: Vec<Pool>,
    full: bool,
}

impl Tournament {
    pub fn new() -> Self {
        let pool_list = utils::make_pools(POOL_AMOUNT);

        Tournament {
            pool_list,
            full: false,
        }
    }
}

pub struct State {
    pub tournament: Option<Tournament>,
}

pub struct ApiResponse {}
