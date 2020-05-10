#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::all)]

use anyhow::Result;
use dotenv::dotenv;
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use std::env;
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct GreeterService {}

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

async fn run() -> Result<()> {
    let addr: SocketAddr = env::var("SOCKET_ADDR")?.parse()?;
    let greeter = GreeterService::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
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
