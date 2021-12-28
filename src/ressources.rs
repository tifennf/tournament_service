use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    core::{PLAYER_AMOUNT, POOL_AMOUNT, POOL_SIZE},
    utils,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DiscordName {
    pub name: String,
    pub tag: String,
}

impl DiscordName {
    pub fn new(name: String, tag: u16) -> Result<Self, &'static str> {
        if tag > 9999 {
            return Err("Invalid discord tag");
        }

        let tag = if tag < 1000 {
            let mut tag = tag.to_string();

            let mut i = 0;

            while tag.len() < 4 {
                tag.insert(i, '0');
                i += 1;
            }

            tag
        } else {
            tag.to_string()
        };

        Ok(Self { name, tag })
    }

    pub fn get_full_name(&self) -> String {
        format!("{}#{}", self.name, self.tag)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Player {
    pub league_name: String,
    pub discord_name: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PlayerVerified {
    pub league_name: String,
    pub discord_name: DiscordName,
}

impl TryFrom<Player> for PlayerVerified {
    type Error = Player;

    fn try_from(value: Player) -> Result<Self, Self::Error> {
        let (name, tag) = {
            let result = &value.discord_name.split('#').collect::<Vec<&str>>();
            if result.len() != 2 {
                return Err(value);
            }

            let tag: u16 = result[1].parse().map_err(|_| value.clone())?;

            (result[0], tag)
        };

        let discord_name = DiscordName::new(name.to_string(), tag).map_err(|_| value.clone())?;

        let player = Self {
            league_name: value.league_name,
            discord_name,
        };

        Ok(player)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pool {
    player_list: Vec<PlayerVerified>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Tournament {
    pool_list: Vec<Pool>,
}

impl Tournament {
    pub fn new(pool_amount: usize) -> Self {
        let pool_list = utils::make_pools(pool_amount);

        Tournament { pool_list }
    }

    pub fn fill(&mut self, player_list: &HashSet<PlayerVerified>) {
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

#[derive(Debug, Clone, Serialize, Default)]
pub struct State {
    pub tournament: Option<Tournament>,
    pub player_list: Option<PlayerList>,
    pub open: bool,
    pub tournament_name: Option<String>,
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
    list: HashSet<PlayerVerified>,
    pub max_amount: PlayerAmount,
    pub current_amount: usize,
}

impl PlayerList {
    pub fn new(max_amount: usize) -> Self {
        let amount = PlayerAmount::new(max_amount).unwrap();
        let list = HashSet::new();
        let current_amount = list.len();

        Self {
            list,
            max_amount: amount,
            current_amount,
        }
    }

    pub fn insert(&mut self, player: PlayerVerified) -> bool {
        let max_len = self.max_amount.0;
        let list_len = self.list.len();

        let condition = list_len < max_len && self.list.insert(player);
        if condition {
            self.current_amount = self.list.len()
        }

        condition
    }

    pub fn list(&self) -> &HashSet<PlayerVerified> {
        &self.list
    }
}

#[derive(Deserialize)]
pub struct InscriptionsState {
    pub open: bool,
}

#[derive(Deserialize)]
pub struct InitTournament {
    pub max_player: usize,
    pub name: String,
}
