#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

#[actix_rt::main]
async fn main() {
    tracing_subscriber::fmt::init();

    patina::main().await;
}
