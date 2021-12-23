use serde::{Deserialize, Serialize};

use crate::utils::make_pools;

const POOL_AMOUNT: u8 = 8;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Tournament {
    pool_list: Vec<Pool>,
    full: bool,
}

impl Tournament {
    pub fn new() -> Self {
        let pool_list = make_pools(POOL_AMOUNT);

        Tournament {
            pool_list,
            full: false,
        }
    }
}
