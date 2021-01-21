use crate::{authentication::ProviderId, http::hal::Link};

impl From<ProviderId> for Link {
    fn from(value: ProviderId) -> Self {
        Self {
            href: format!("/authentication/{}", value.0),
            name: Some(value.0),
        }
    }
}
