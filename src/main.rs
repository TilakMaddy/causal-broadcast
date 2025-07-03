use axum::{Router, routing::post};
use clap::Parser;
use std::{
    sync::{Arc, RwLock},
    usize,
};
use tracing_subscriber::EnvFilter;

mod app;
mod consensus;
mod serve;
mod system;

use app::*;
use consensus::*;
use system::*;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    node_id: usize,
}

#[tokio::main]
async fn main() {
    let cli_args = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let shared_state = FullSystemState::new_in_rwlock(cli_args.node_id);

    // routes
    let app = Router::new()
        .route("/broadcast", post(serve::broadcast_message))
        .route("/receive", post(serve::receive_message))
        .with_state(shared_state);

    // listen
    let address = format!("0.0.0.0:{}", 3000 + cli_args.node_id);
    let listener = tokio::net::TcpListener::bind(address.clone())
        .await
        .unwrap();

    // serve
    tracing::trace!("listening on {}", address);
    axum::serve(listener, app).await.unwrap();
}
