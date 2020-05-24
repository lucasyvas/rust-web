#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::all)]

mod core;
mod grpc;

use anyhow::Result;
use dotenv::dotenv;
use std::env;
use tonic::transport::Server;

async fn run() -> Result<()> {
    #[rustfmt::skip]
    let pool = core::database::create_pool(#[cfg(not(test))] env::var("DATABASE_URL")?).await?;
    core::database::create_schema(pool.as_ref()).await?;

    let checklist_model = core::checklist::model::Model::new(pool.clone());
    let checklist_service = core::checklist::service::Service::new(checklist_model);
    let checklist_router = grpc::checklist::Router::new(checklist_service.clone());

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
