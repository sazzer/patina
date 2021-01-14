/// The actual service layer.
pub struct Service {}

impl Service {
    /// Create a new instance of the service layer.
    pub fn new() -> Self {
        tracing::info!("Building service");
        tracing::info!("Built service");
        Self {}
    }

    /// Start the service listening on the given HTTP port.
    ///
    /// # Parameters
    /// - `port` - The port number to listen on
    pub async fn start(self, port: u16) {
        tracing::info!(port = port, "Starting service");
    }
}
