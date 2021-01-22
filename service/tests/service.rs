use actix_http::Request;
use patina::testing::TestResponse;
use patina_testdatabase::seed::SeedData;

/// Wrapper around the service to test.
pub struct Service {
    service: patina::Service,

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
            google:   Some(patina::GoogleSettings {
                client_id:     "GoogleClientId".to_string(),
                client_secret: "GoogleClientSecret".to_string(),
                redirect_url:  "http://example.com/authentication/google/redirect".to_string(),
                auth_url:      None,
                token_url:     None,
            }),
        };
        let service = patina::Service::new(&settings).await;

        Self { service, database }
    }

    /// Inject a request into the service and get the response back.
    pub async fn inject(&self, req: Request) -> TestResponse {
        self.service.inject(req).await
    }

    /// Seed some data into the database
    ///
    /// # Parameters
    /// - `data` - The data to seed
    pub async fn seed(&self, data: &dyn SeedData) -> &Self {
        self.database.seed(data).await;
        self
    }
}
