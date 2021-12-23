use axum::{response::IntoResponse, Json};

use crate::ressources::Player;

pub async fn root() -> &'static str {
    "test ok"
}

pub async fn register_player(Json(payload): Json<Player>) -> impl IntoResponse {
    format!("{:#?}", payload)
}

pub async fn print_tournament() {}
