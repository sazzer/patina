use actix_http::Request;
use patina::testing::TestResponse;

/// Wrapper around the service to test.
pub struct Service {
    service:  patina::Service,
    database: patina_testdatabase::TestDatabase,
}

impl Service {
    /// Create a new test service.
    pub fn new() -> Self {
        let database = patina_testdatabase::TestDatabase::default();

        let settings = patina::Settings {};
        let service = patina::Service::new(&settings);

        Self { service, database }
    }

    /// Inject a request into the service and get the response back.
    pub async fn inject(&self, req: Request) -> TestResponse {
        self.service.inject(req).await
    }
}
