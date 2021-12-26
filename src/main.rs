use std::net::SocketAddr;

pub mod core;
pub mod middlewares;
pub mod ressources;
pub mod routes;
pub mod utils;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "tournament_service=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("Listening on address: {}", addr);

    core::run(&addr).await;
}
