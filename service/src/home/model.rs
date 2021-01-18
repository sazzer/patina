use crate::http::hal::{HalPayload, Link};

/// The details needed for the home document.
pub struct HomeDocument {
    /// The payload of the home document.
    pub payload: HalPayload<()>,
}

/// Trait that components able to contribute to the home document can implement.
pub trait Contributor: Send + Sync {
    /// Get the links that this component is contributing.
    fn get_links(&self) -> Vec<(&'static str, Link)>;
}
