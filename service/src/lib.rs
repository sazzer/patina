#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::future_not_send)]

mod health;
mod server;
mod service;

pub use service::{Service, Settings};
