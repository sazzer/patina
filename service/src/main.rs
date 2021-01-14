#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

use dotenv::dotenv;

#[actix_rt::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    patina::main().await;
}
