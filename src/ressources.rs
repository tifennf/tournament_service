use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{utils, PLAYER_AMOUNT, POOL_AMOUNT, POOL_SIZE};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Player {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pool {
    player_list: Vec<Player>,
    id: usize,
    amount: PoolAmount,
}

impl Pool {
    pub fn new(id: usize, amount: usize) -> Pool {
        let amount = PoolAmount(amount);

        Pool {
            player_list: Vec::new(),
            id,
            amount,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tournament {
    pool_list: Vec<Pool>,
}

impl Tournament {
    pub fn new(pool_amount: usize) -> Self {
        let pool_list = utils::make_pools(pool_amount);

        Tournament { pool_list }
    }

    pub fn fill(&mut self, player_list: &HashSet<Player>) {
        let player_list = player_list.clone().into_iter().collect();

        let player_list = utils::shuffle_players(player_list);

        let pool_list = self
            .pool_list
            .clone()
            .into_iter()
            .zip(player_list.chunks(POOL_SIZE))
            .map(|(mut pool, player_list)| {
                let list = &mut pool.player_list;
                while list.len() < pool.amount.0 {
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
    pub player_list: Option<PlayerList>,
    pub open: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            tournament: None,
            player_list: None,
            open: false,
        }
    }
}

pub trait RequestChecker {
    fn check(&self) -> bool;
}

#[derive(Debug, Clone, Serialize)]

pub struct PlayerAmount(pub usize);

impl PlayerAmount {
    pub fn new(amount: usize) -> Result<Self, &'static str> {
        if !PLAYER_AMOUNT.contains(&amount) {
            return Err("Invalid player amount");
        }

        Ok(Self(amount))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolAmount(usize);

impl PoolAmount {
    pub fn new(amount: usize) -> Result<Self, &'static str> {
        if !POOL_AMOUNT.contains(&amount) {
            return Err("Invalid pool amount");
        }

        Ok(Self(amount))
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerList {
    player_list: HashSet<Player>,
    pub max_amount: PlayerAmount,
}

impl PlayerList {
    pub fn new(max_amount: usize) -> Self {
        let amount = PlayerAmount::new(max_amount).unwrap();
        let player_list = HashSet::new();

        Self {
            player_list,
            max_amount: amount,
        }
    }

    pub fn insert(&mut self, player: Player) -> bool {
        let max_len = self.max_amount.0;
        let list_len = self.player_list.len();

        list_len < max_len && self.player_list.insert(player)
    }

    pub fn list(&self) -> &HashSet<Player> {
        &self.player_list
    }
}
