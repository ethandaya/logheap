use clickhouse::Client;

pub fn init_client() -> Client {
    Client::default()
        .with_url("http://localhost:8123")
}