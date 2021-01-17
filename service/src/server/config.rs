use std::sync::Arc;

use super::{Configurer, Server};

/// Configuration component for the HTTP Server.
pub struct Builder {
    configurers: Vec<Arc<dyn Configurer>>,
}

/// Create a builder used to construct the server.
pub fn builder() -> Builder {
    Builder {
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
        Server::new(self.configurers)
    }
}
