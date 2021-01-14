use crate::server::Server;

/// The actual service layer.
pub struct Service {
    server: Server,
}

/// The settings needed to build the service.
#[derive(Debug)]
pub struct Settings {}

impl Service {
    /// Create a new instance of the service layer.
    #[must_use]
    pub fn new(_settings: &Settings) -> Self {
        tracing::info!("Building service");

        let server = crate::server::config::Component::default().build();

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
