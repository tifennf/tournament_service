use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{AddExtensionLayer, Json, Router};
use serde::Serialize;
use tower_http::trace::TraceLayer;

use crate::ressources::{PlayerList, Tournament};
use crate::routes::not_found;
use crate::routes::{info, manage_tournament, root};

pub async fn run(addr: &SocketAddr) {
    let state = Arc::new(Mutex::new(State {
        tournament: None,
        player_list: None,
        tournament_name: None,
        open: false,
    }));

    let app = Router::new()
        .merge(manage_tournament())
        .merge(info())
        .merge(root())
        .fallback(get(not_found))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(state));

    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub const POOL_AMOUNT: [usize; 4] = [1, 2, 4, 8];
pub const PLAYER_AMOUNT: [usize; 4] = [8, 16, 32, 64];
pub const POOL_SIZE: usize = 8;

pub type SharedState = Arc<Mutex<State>>;

#[derive(Debug, Clone)]
pub struct ApiResponse<D> {
    inner: Json<ApiResponseInner<D>>,
}

impl<D> ApiResponse<D> {
    pub fn new(status: StatusCode, data: D) -> Self {
        let status = status.as_u16();

        Self {
            inner: Json(ApiResponseInner { status, data }),
        }
    }
}

impl<D: Serialize> IntoResponse for ApiResponse<D> {
    fn into_response(self) -> axum::response::Response {
        let status = self.inner.0.status;
        let mut response = self.inner.into_response();

        *response.status_mut() = StatusCode::from_u16(status).unwrap();

        response
    }
}

#[derive(Debug, Serialize, Clone)]
struct ApiResponseInner<D> {
    status: u16,
    data: D,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct State {
    pub tournament: Option<Tournament>,
    pub player_list: Option<PlayerList>,
    pub open: bool,
    pub tournament_name: Option<String>,
}
