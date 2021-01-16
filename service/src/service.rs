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
    pub async fn new(settings: &Settings) -> Self {
        tracing::info!("Building service");

        let database = crate::database::config::new(&settings.database).await;
        let users = crate::users::config::new(database.clone());
        let health = crate::health::config::builder().with_component("db", database).build();
        let server = crate::server::config::builder().with_component(health).with_component(users).build();

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
