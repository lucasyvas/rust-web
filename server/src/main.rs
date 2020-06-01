#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![deny(warnings)]

mod core;
mod graphql;
mod grpc;

use anyhow::Result;
use dotenv::dotenv;

use juniper::{
    http::{graphiql::graphiql_source, GraphQLRequest},
    EmptySubscription, RootNode,
};

use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use tonic::transport::Server;
use warp::Filter;

type GraphqlSchema = RootNode<
    'static,
    graphql::checklist::Query,
    graphql::checklist::Mutation,
    EmptySubscription<graphql::checklist::Context>,
>;

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

async fn run() -> Result<()> {
    run_grpc().await?;
    Ok(())
}

#[allow(dead_code)]
async fn run_grpc() -> Result<()> {
    let pool = core::database::create_pool(&env::var("DATABASE_URL")?).await?;
    core::database::create_schema(&pool).await?;

    let checklist_model = core::checklist::model::Model::new(pool.clone());
    let checklist_service = core::checklist::service::Service::new(checklist_model);
    let checklist_controller = grpc::checklist::Controller::new(checklist_service.clone());

    Server::builder()
        .add_service(checklist_controller)
        .serve(env::var("SOCKET_ADDR")?.parse()?)
        .await?;

    Ok(())
}

#[allow(dead_code)]
async fn run_graphql() -> Result<()> {
    use graphql::checklist::{Context, Mutation, Query};

    let checklist_schema =
        warp::any().map(move || GraphqlSchema::new(Query, Mutation, EmptySubscription::new()));
    let checklist_context = warp::any().map(move || Context {});

    let graphql_route = warp::post()
        .and(warp::path!("graphql"))
        .and(checklist_schema.clone())
        .and(checklist_context.clone())
        .and(warp::body::json())
        .and_then(graphql);

    let graphiql_route = warp::get()
        .and(warp::path!("graphiql"))
        .map(|| warp::reply::html(graphiql_source("graphql", None)));

    let routes = graphql_route.or(graphiql_route);

    warp::serve(routes)
        .run(env::var("SOCKET_ADDR")?.parse::<SocketAddr>()?)
        .await;

    Ok(())
}

async fn graphql(
    schema: GraphqlSchema,
    ctx: graphql::checklist::Context,
    req: GraphQLRequest,
) -> Result<impl warp::Reply, Infallible> {
    let res = req.execute(&schema, &ctx).await;
    let json = serde_json::to_string(&res).expect("Invalid JSON response");
    Ok(json)
}
