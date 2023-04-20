use std::convert::Infallible;
use serde_json::json;


pub async fn ingest_event() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::json(&json!({
    "name": "Ethan"
})))
}