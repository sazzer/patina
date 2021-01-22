#![deny(clippy::all, clippy::pedantic, clippy::nursery)]

use config::{Config, Environment};
use dotenv::dotenv;
use serde::Deserialize;

/// Representation of the application settings that will be loaded from the environment
#[derive(Debug, Deserialize)]
struct Settings {
    /// The port on which the HTTP server should listen on
    pub port: Option<u16>,

    /// The URL to use to connect to the database
    pub database_url: String,

    /// Client ID to use for authentication with Google
    pub google_client_id:     Option<String>,
    /// Client Secret to use for authentication with Google
    pub google_client_secret: Option<String>,
    /// URL to redirect the user back to after authenticating with Google
    pub google_redirect_url:  Option<String>,
    /// URL pattern to start authenticating with Google
    pub google_auth_url:      Option<String>,
    /// URL to call to get an access token from Google
    pub google_token_url:     Option<String>,
}

impl Default for Settings {
    /// Construct the settings from the environment
    ///
    /// # Returns
    /// The Settings object, loaded from the environment variables
    fn default() -> Self {
        let mut s = Config::new();
        s.merge(Environment::default())
            .expect("Failed to load environment properties");

        s.try_into().expect("Failed to build settings from config")
    }
}

#[actix_rt::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let settings = Settings::default();
    tracing::debug!(settings = ?settings, "Loaded settings");

    let service = patina::Service::new(&patina::Settings {
        database: patina::DatabaseSettings {
            url: settings.database_url,
        },
        google:   match (
            settings.google_client_id,
            settings.google_client_secret,
            settings.google_redirect_url,
        ) {
            (Some(client_id), Some(client_secret), Some(redirect_url)) => {
                Some(patina::GoogleSettings {
                    client_id,
                    client_secret,
                    redirect_url,
                    auth_url: settings.google_auth_url,
                    token_url: settings.google_token_url,
                })
            },
            _ => None,
        },
    })
    .await;
    service.start(settings.port.unwrap_or(8000)).await;
}
