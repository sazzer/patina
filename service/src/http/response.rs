use actix_http::{
    http::header::CacheControl,
    http::{
        header::{CacheDirective, ContentType, ETag, EntityTag},
        StatusCode,
    },
    Error, Response as HttpResponse,
};
use actix_web::Responder;
use futures::future::{ok, Ready};
use serde::Serialize;

/// Friendly representation of the HTTP Response to send back.
pub struct Response<T>
where
    T: Serialize,
{
    /// The HTTP Status Code
    pub status: StatusCode,
    /// The caching controls
    pub cache_control: Vec<CacheDirective>,
    /// The content type
    pub content_type: ContentType,
    /// The etag to return, if applicable
    pub etag: Option<EntityTag>,
    /// The actual response body
    pub body: Option<T>,
}

impl<T> Default for Response<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self {
            status: StatusCode::OK,
            cache_control: vec![CacheDirective::NoCache],
            content_type: ContentType::json(),
            etag: None,
            body: None,
        }
    }
}

impl<T> Responder for Response<T>
where
    T: Serialize,
{
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> Self::Future {
        let mut response = HttpResponse::build(self.status);

        response.set(CacheControl(self.cache_control));
        response.set(self.content_type);

        if let Some(etag) = self.etag {
            response.set(ETag(etag));
        }

        let built = response.json(self.body);

        ok(built)
    }
}
