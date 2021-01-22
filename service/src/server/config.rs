use std::sync::Arc;

use prometheus::Registry;

use super::{Configurer, Server};

/// Configuration component for the HTTP Server.
pub struct Builder {
    prometheus:  Registry,
    configurers: Vec<Arc<dyn Configurer>>,
}

/// Create a builder used to construct the server.
pub fn builder(prometheus: Registry) -> Builder {
    Builder {
        prometheus,
        configurers: vec![],
    }
}

impl Builder {
    /// Add a new component to the server.
    ///
    /// # Parameters
    /// - `component` - The component to add
    pub fn with_component(mut self, component: Arc<dyn Configurer>) -> Self {
        self.configurers.push(component);
        self
    }

    /// Build the HTTP Server.
    pub fn build(self) -> Server {
        Server::new(self.configurers, self.prometheus)
    }
}
