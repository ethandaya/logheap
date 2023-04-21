use crate::handlers::issues as issues_handler;
use clickhouse::Client;
use serde::Deserialize;
use warp::Filter;

#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
    pub addresses: Option<String>,
    pub transactions: Option<String>,
}

pub fn issues(
    db: Client,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    list_issues(db.clone())
}

pub fn list_issues(
    db: Client,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("issues")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(warp::any().map(move || db.clone()))
        .and_then(issues_handler::list_issues)
}
