use std::convert::Infallible;

use clickhouse::Client;
use clickhouse::Row;
use serde::{Deserialize, Serialize};

use crate::filters::ListOptions;

#[derive(Row, Deserialize, Serialize)]
pub struct Log {
    pub source: String,
}

pub async fn list_issues(opts: ListOptions, db: Client) -> Result<impl warp::Reply, Infallible> {
    let mut query = String::from("SELECT source FROM logs WHERE 1=1");

    let addresses: Vec<String> = if let Some(addresses) = opts.addresses {
        addresses.split(",").map(|s| s.to_string()).collect()
    } else {
        vec![]
    };

    println!("addresses: {:?}", addresses);

    let transactions: Vec<String> = if let Some(transactions) = opts.transactions {
        transactions.split(",").map(|s| s.to_string()).collect()
    } else {
        vec![]
    };

    if addresses.len() > 0 {
        query.push_str(" AND arrayExists(x -> has(accounts, Lower(x)), ?)");
    }

    if transactions.len() > 0 {
        query.push_str(" AND arrayExists(x -> has(transactions, Lower(x)), ?)");
    }

    query.push_str(" ORDER BY _timestamp LIMIT ? OFFSET ?");

    println!("query: {}", query);

    let mut query = db.query(&query);

    if addresses.len() > 0 {
        query = query.bind(&addresses);
    }

    if transactions.len() > 0 {
        query = query.bind(&transactions);
    }

    query = query
        .bind(opts.limit.unwrap_or(100))
        .bind(opts.offset.unwrap_or(0));

    let result = query.fetch_all::<Log>().await;

    let todos = match result {
        Ok(rows) => rows,
        Err(e) => {
            println!("Error: {}", e);
            vec![]
        }
    };

    Ok(warp::reply::json(&todos))
}
