use axum::{routing::MethodRouter, Router};

use crate::ressources::Pool;

pub fn make_pools(number: u8) -> Vec<Pool> {
    (0..number).into_iter().map(|n| Pool::new(n)).collect()
}

pub fn route(path: &str, method_router: MethodRouter) -> Router {
    Router::new().route(path, method_router)
}
