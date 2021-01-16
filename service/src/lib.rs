#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::future_not_send, clippy::module_name_repetitions)]

mod database;
mod health;
mod http;
mod model;
mod server;
mod service;
mod users;

pub use database::config::Settings as DatabaseSettings;
pub use service::{testing, Service, Settings};
