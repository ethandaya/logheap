#![deny(warnings)]

use std::env;
use warp::Filter;
extern crate pretty_env_logger;

mod handlers;
mod filters;


#[tokio::main]
async fn main() {

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "ingest=info");
    }

    pretty_env_logger::init();

    let health = warp::path::end().map(|| "OK");
    let api = filters::ingest();

    let api_routes = api.with(warp::log("ingest"));
    let routes = health.or(api_routes);

    println!("Listening");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}