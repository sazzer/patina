#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::future_not_send, clippy::module_name_repetitions)]

mod health;
mod http;
mod server;
mod service;

pub use service::{Service, Settings};
