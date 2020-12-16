use clap::Clap;
use hyper::server::Server;
use listenfd::ListenFd;
use std::convert::Infallible;
use warp::{http::Method, Filter};

use kerria::{
    api,
    environment::{Args, Environment},
    helpers::problem,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    if dotenv::dotenv().is_err() {
        eprintln!("Warning: Did not find .env file in current working directory!");
    }
    let args = Args::parse();
    let env = Environment::new(&args).await?;
    // let env = warp::any().map(move || env.clone());
    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_header("content-type")
        .allow_header("authorization")
        .allow_any_origin()
        .build();
    let log = warp::log("api::request");
    let status = api::status();
    let cosmetics = api::cosmetics(env.clone());
    let admin_filters = api::admin_filters(env.clone());

    let svc = warp::service(
        status
            .or(cosmetics)
            .or(admin_filters)
            .recover(problem::unpack)
            .with(cors)
            .with(log),
    );

    let make_svc = hyper::service::make_service_fn(|_: _| {
        let svc = svc.clone();
        async move { Ok::<_, Infallible>(svc) }
    });

    let mut listenfd = ListenFd::from_env();

    let server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        Server::from_tcp(l)?
    } else {
        Server::bind(&args.host)
    };

    server.serve(make_svc).await?;

    Ok(())
}
