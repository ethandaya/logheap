use rskafka::client::{Client, ClientBuilder};

pub fn init_kafka() -> Client {
    let connection = bootstrap_url.to_owned();
    let client = ClientBuilder::new(vec![connection]).build().await.unwrap();
    client
}
