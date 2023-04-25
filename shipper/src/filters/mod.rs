use crate::handlers::ingest as ingest_handler;
use rskafka::client::Client;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

pub fn ingest(
    kafka: Arc<Mutex<Client>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    ingest_event(kafka)
}

pub fn ingest_event(
    kafka: Arc<Mutex<Client>>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("ingest")
        .and(warp::post())
        .and(warp::any().map(move || kafka.clone()))
        .and(warp::body::bytes())
        .and_then(
            move |kafka: Arc<Mutex<Client>>, bytes: warp::hyper::body::Bytes| {
                ingest_handler::ingest_event(kafka, bytes.to_vec())
            },
        )
}
