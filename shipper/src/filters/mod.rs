use warp::Filter;
use crate::handlers::{ingest as ingest_handler};

pub fn ingest() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
    ingest_event()
}

pub fn ingest_event() -> impl Filter<Extract=(impl warp::Reply, ), Error=warp::Rejection> + Clone {
    warp::path!("ingest")
        .and(warp::post())
        .and_then(ingest_handler::ingest_event)
}