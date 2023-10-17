mod endpoints;
mod users;
pub use endpoints::Endpoint;
use futures::{stream::FuturesUnordered, TryStreamExt};
pub use users::User;

const DB_ADDR: &str = "https://k0oarenehd.europe-west4.gcp.clickhouse.cloud:8443";

#[derive(Clone)]
pub struct Db {
    client: clickhouse::Client,
}
pub async fn connect() -> Db {
    let client = clickhouse::Client::default()
        .with_url(DB_ADDR)
        .with_user("default")
        .with_password("DVuRkT_2ubHMr")
        .with_database("latency_watcher");
    [Endpoint::CREATE_TABLE, User::CREATE_TABLE]
        .iter()
        .map(|q| client.query(q).execute())
        .collect::<FuturesUnordered<_>>()
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    let db = Db { client };
    db.new_user(User {
        id: "nikos".to_owned(),
        password: "nikos".to_owned(),
        group: "default".to_owned(),
        is_admin: true,
        permissions: 0,
        theme: "crimson".to_string(),
    })
    .await
    .ok();
    db
}
