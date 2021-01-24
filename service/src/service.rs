pub mod testing;

use prometheus::Registry;

use crate::server::Server;

/// The actual service layer.
pub struct Service {
    server: Server,
}

/// The settings needed to build the service.
#[derive(Debug)]
pub struct Settings {
    pub database: crate::database::config::Settings,
    pub google:   Option<crate::authentication::config::GoogleSettings>,
}

impl Service {
    /// Create a new instance of the service layer.
    #[tracing::instrument]
    pub async fn new(settings: &Settings) -> Self {
        tracing::info!("Building service");

        let prometheus = Registry::new();

        let database = crate::database::config::new(&settings.database, &prometheus).await;
        let _authorization = crate::authorization::config::new();
        let users = crate::users::config::new(database.clone());
        let authentication = crate::authentication::config::builder()
            .with_google(&settings.google)
            .build();
        let health = crate::health::config::builder()
            .with_component("db", database)
            .build(&prometheus);
        let home = crate::home::config::builder()
            .with_component(health.clone())
            .with_component(authentication.clone())
            .build();
        let server = crate::server::config::builder(prometheus)
            .with_component(health)
            .with_component(users)
            .with_component(authentication)
            .with_component(home)
            .build();

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
