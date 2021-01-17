use actix_http::Request;
use patina::testing::TestResponse;

/// Wrapper around the service to test.
pub struct Service {
    service: patina::Service,

    #[allow(dead_code)] // TODO: FIX
    database: patina_testdatabase::TestDatabase,
}

impl Service {
    /// Create a new test service.
    pub async fn new() -> Self {
        let _ = tracing_subscriber::fmt::try_init();

        let database = patina_testdatabase::TestDatabase::default();

        let settings = patina::Settings {
            database: patina::DatabaseSettings {
                url: database.url.clone(),
            },
        };
        let service = patina::Service::new(&settings).await;

        Self { service, database }
    }

    /// Inject a request into the service and get the response back.
    pub async fn inject(&self, req: Request) -> TestResponse {
        self.service.inject(req).await
    }
}
