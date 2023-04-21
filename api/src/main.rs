#![deny(warnings)]

use std::env;
use warp::Filter;
extern crate pretty_env_logger;

mod handlers;
mod filters;
mod database;


#[tokio::main]
async fn main() {

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "ingest=info");
    }

    pretty_env_logger::init();

    let db = database::init_client();

    let health = warp::path::end().map(|| "OK");
    let issues = filters::issues(db);

    let api_routes = issues.with(warp::log("issues"));
    let routes = health.or(api_routes);

    println!("Listening");
    warp::serve(routes).run(([127, 0, 0, 1], 8081)).await;
}