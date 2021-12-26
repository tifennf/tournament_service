use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{utils, POOL_AMOUNT, POOL_MAX_SIZE};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Player {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pool {
    player_list: Vec<Player>,
    id: usize,
    max_size: usize,
}

impl Pool {
    pub fn new(id: usize, max_size: usize) -> Pool {
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

    pub fn fill(&mut self, player_list: HashSet<Player>) {
        let player_list = player_list.clone().into_iter().collect();

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

#[derive(Debug, Clone, Serialize)]
pub struct State {
    pub tournament: Option<Tournament>,
    pub player_list: HashSet<Player>,
    pub open: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            tournament: None,
            player_list: Default::default(),
            open: false,
        }
    }
}

pub struct ApiResponse {}

pub trait RequestChecker {
    fn check(&self) -> bool;
}
