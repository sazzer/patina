#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

#[actix_rt::main]
async fn main() {
    patina::main().await;
}
