#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::all)]

mod checklist;

use anyhow::Result;
use dotenv::dotenv;
#[cfg(not(test))]
use sqlx::SqlitePool;
use std::env;
#[cfg(not(test))]
use std::sync::Arc;
use tonic::transport::Server;

async fn run() -> Result<()> {
    #[cfg(not(test))]
    let pool = Arc::new(SqlitePool::builder().min_size(1).build("sqlite:").await?);

    #[rustfmt::skip]
    let checklist_model = checklist::model::Model::new(#[cfg(not(test))]pool.clone());
    let checklist_service = checklist::service::Service::new(checklist_model.clone());
    let checklist_router = checklist::router::Router::new(checklist_service.clone());

    Server::builder()
        .add_service(checklist_router)
        .serve(env::var("SOCKET_ADDR")?.parse()?)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let err = match run().await {
        Ok(_) => return,
        Err(err) => err,
    };

    log::error!("{:?}", err);
}
