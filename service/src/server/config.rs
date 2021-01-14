use super::{Configurer, Server};
use std::sync::Arc;

/// Configuration component for the HTTP Server.
#[derive(Default)]
pub struct Component {
    configurers: Vec<Arc<dyn Configurer>>,
}

impl Component {
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
