mod api;
mod db;
mod error;
pub use error::{Error, Result};
async fn run() {
    let db = db::connect().await;
    api::start(db).await;
}

#[tokio::main]
async fn main() {
    run().await;
}
