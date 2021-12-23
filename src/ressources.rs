use core::slice;

use serde::{Deserialize, Serialize};

use crate::utils;

const POOL_AMOUNT: u8 = 8;
const POOL_MAX_SIZE: usize = 8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pool {
    player_list: Vec<Player>,
    id: u8,
    max_size: usize,
}

impl Pool {
    pub fn new(id: u8, max_size: usize) -> Pool {
        Pool {
            player_list: Vec::new(),
            id,
            max_size,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tournament {
    pool_list: Vec<Pool>,
}

impl Tournament {
    pub fn new() -> Self {
        let pool_list = utils::make_pools(POOL_AMOUNT, POOL_MAX_SIZE);

        Tournament { pool_list }
    }

    pub fn fill(&mut self, player_list: Vec<Player>) {
        let player_list = utils::shuffle_players(player_list);

        let pool_list = self
            .pool_list
            .clone()
            .into_iter()
            .zip(player_list.chunks(POOL_MAX_SIZE))
            .map(|(mut pool, player_list)| {
                let list = &mut pool.player_list;
                while list.len() < pool.max_size {
                    for player in player_list {
                        list.push(player.clone())
                    }
                }

                pool
            })
            .collect::<Vec<Pool>>();

        self.pool_list = pool_list;
    }
}

pub struct State {
    pub tournament: Option<Tournament>,
    pub player_list: Vec<Player>,
}

pub struct ApiResponse {}
