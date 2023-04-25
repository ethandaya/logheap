use std::collections::BTreeMap;
use std::sync::Arc;

use rskafka::client::partition::Compression::Gzip;
use rskafka::client::Client;
use rskafka::record::Record;
use rskafka::time::OffsetDateTime;
use serde_json::Value;
use tokio::sync::Mutex;

use uuid::Uuid;

pub async fn ingest_event(
    kafka: Arc<Mutex<Client>>,
    body: Vec<u8>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let client = kafka.lock().await;
    let partition_client = match client.partition_client("log".to_owned(), 0) {
        Ok(client) => client,
        Err(_) => {
            return Ok(warp::reply::with_status(
                "ERROR",
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    };

    let key = Uuid::new_v4().to_string().into_bytes();
    // Decode the JSON string from the received body
    let decoded_json: Value = serde_json::from_slice(&body).unwrap();

    // Convert the decoded JSON Value back to a Vec<u8>
    let body_unescaped = serde_json::to_vec(&decoded_json).unwrap();

    let record = Record {
        key: Some(key),
        value: Some(body_unescaped),
        headers: BTreeMap::new(),
        timestamp: OffsetDateTime::now_utc(),
    };
    match partition_client.produce(vec![record], Gzip).await {
        Ok(_) => (),
        Err(_) => {
            return Ok(warp::reply::with_status(
                "ERROR",
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }

    Ok(warp::reply::with_status("OK", warp::http::StatusCode::OK))
}
