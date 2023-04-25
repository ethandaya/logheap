use rskafka::client::{Client, ClientBuilder};

pub async fn init_client(bootstrap_url: &str) -> Client {
    let connection = bootstrap_url.to_owned();
    let client = ClientBuilder::new(vec![connection]).build().await.unwrap();
    client
}
