use std::collections::BTreeMap;

use actix_http::http::{
    header::{CacheDirective, ContentType, EntityTag},
    StatusCode,
};
use serde::Serialize;

use super::response::Response;

/// Representation of a single link in a resource.
#[derive(Debug, Serialize, Clone)]
pub struct Link {
    /// The actual link href.
    pub href: String,
    /// The optional name of the link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl<S> From<S> for Link
where
    S: Into<String>,
{
    fn from(href: S) -> Self {
        Self {
            href: href.into(),
            name: None,
        }
    }
}

/// Representation of a set of links for a single name.
#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum Links {
    /// A single link.
    Single(Link),
    /// A set of multiple links.
    Multiple(Vec<Link>),
}

/// Representation of a HAL resource.
#[derive(Debug, Serialize, Clone)]
pub struct HalPayload<T> {
    /// The data for the resource
    #[serde(flatten)]
    pub data:  T,
    /// The links for the resource
    #[serde(rename = "_links", skip_serializing_if = "BTreeMap::is_empty")]
    pub links: BTreeMap<String, Links>,
}

impl<T> HalPayload<T> {
    /// Create a new HAL Resource.
    ///
    /// # Parameters
    /// - `data` - The data for the resource
    #[allow(clippy::missing_const_for_fn)]
    pub fn new(data: T) -> Self {
        Self {
            data,
            links: BTreeMap::new(),
        }
    }

    /// Add a link to the resource
    ///
    /// # Parameters
    /// - `name` - The name of the link
    /// - `link` - The actual link
    pub fn with_link<N, L>(mut self, name: N, link: L) -> Self
    where
        N: Into<String>,
        L: Into<Link>,
    {
        let name = name.into();
        let link = link.into();

        let links = match self.links.remove(&name) {
            None => Links::Single(link),
            Some(Links::Single(previous)) => Links::Multiple(vec![previous, link]),
            Some(Links::Multiple(mut previous)) => {
                previous.push(link);
                Links::Multiple(previous)
            },
        };

        self.links.insert(name, links);

        self
    }
}

/// The actual resource representation for a HAL Response.
#[derive(Debug, Serialize)]
pub struct HalResponse<T> {
    /// The body of the response
    pub body: Option<HalPayload<T>>,

    /// The status code
    #[serde(skip)]
    pub status: StatusCode,

    /// The caching controls
    #[serde(skip)]
    pub cache_control: Vec<CacheDirective>,

    /// The etag
    #[serde(skip)]
    pub etag: Option<EntityTag>,
}

impl<T> Default for HalResponse<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self {
            body:          None,
            status:        StatusCode::OK,
            cache_control: vec![CacheDirective::NoCache],
            etag:          None,
        }
    }
}

#[allow(clippy::fallible_impl_from)]
impl<I, O> From<I> for Response<HalPayload<O>>
where
    I: Into<HalResponse<O>>,
    O: Serialize,
{
    fn from(input: I) -> Self {
        let content_type = ContentType("application/hal+json".parse().unwrap());

        let hal_response: HalResponse<O> = input.into();
        Self {
            content_type,
            body: hal_response.body,
            status: hal_response.status,
            cache_control: hal_response.cache_control,
            etag: hal_response.etag,
        }
    }
}

#[cfg(test)]
mod tests {
    use assert2::{check, let_assert};

    use super::*;

    #[test]
    fn hal_payload_given_no_links_has_correct_links() {
        let resource = HalPayload::new(());

        check!(resource.links.len() == 0);
    }

    #[test]
    fn hal_payload_given_one_link_has_correct_links() {
        let resource = HalPayload::new(()).with_link("self", "/hello");

        check!(resource.links.len() == 1);

        let_assert!(Some(Links::Single(self_link)) = resource.links.get("self"));
        check!(self_link.href == "/hello");
        check!(self_link.name == None);
    }

    #[test]
    fn hal_payload_given_two_links_with_different_names_has_correct_links() {
        let resource = HalPayload::new(()).with_link("self", "/hello").with_link("other", "/other");

        check!(resource.links.len() == 2);

        let_assert!(Some(Links::Single(self_link)) = resource.links.get("self"));
        check!(self_link.href == "/hello");
        check!(self_link.name == None);

        let_assert!(Some(Links::Single(other_link)) = resource.links.get("other"));
        check!(other_link.href == "/other");
        check!(other_link.name == None);
    }

    #[test]
    fn hal_payload_given_two_links_with_same_name_has_correct_links() {
        let resource = HalPayload::new(()).with_link("item", "/hello").with_link("item", "/other");

        check!(resource.links.len() == 1);

        let_assert!(Some(Links::Multiple(items)) = resource.links.get("item"));

        let_assert!(Some(first) = items.get(0));
        check!(first.href == "/hello");
        check!(first.name == None);

        let_assert!(Some(second) = items.get(1));
        check!(second.href == "/other");
        check!(second.name == None);
    }
}
