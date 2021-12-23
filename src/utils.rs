use axum::{routing::MethodRouter, Router};
use rand::{prelude::SliceRandom, thread_rng};

use crate::ressources::{Player, Pool};

pub fn make_pools(number: u8, max_size: usize) -> Vec<Pool> {
    (0..number)
        .into_iter()
        .map(|n| Pool::new(n, max_size))
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
