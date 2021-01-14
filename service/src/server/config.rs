use super::{Configurer, Server};
use std::sync::Arc;

/// Configuration component for the HTTP Server.
#[derive(Default)]
pub struct Component {
    configurers: Vec<Arc<dyn Configurer>>,
}

impl Component {
    /// Build the HTTP Server.
    pub fn build(self) -> Server {
        Server::new(self.configurers)
    }
}
