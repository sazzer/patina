pub mod testing;

use crate::server::Server;

/// The actual service layer.
pub struct Service {
    server: Server,
}

/// The settings needed to build the service.
#[derive(Debug)]
pub struct Settings {
    pub database: crate::database::config::Settings,
}

impl Service {
    /// Create a new instance of the service layer.
    #[must_use]
    pub async fn new(settings: &Settings) -> Self {
        tracing::info!("Building service");

        let _database = crate::database::config::new(&settings.database).await;
        let health = crate::health::config::builder().build();
        let server = crate::server::config::builder().with_component(health).build();

        tracing::info!("Built service");

        Self { server }
    }

    /// Start the service listening on the given HTTP port.
    ///
    /// # Parameters
    /// - `port` - The port number to listen on
    pub async fn start(self, port: u16) {
        self.server.start(port).await
    }
}
