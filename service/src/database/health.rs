use async_trait::async_trait;

use super::Database;
use crate::health::HealthCheckable;

#[async_trait]
impl HealthCheckable for Database {
    async fn check_health(&self) -> Result<(), String> {
        let conn = self.try_checkout().await.map_err(|e| e.to_string())?;

        conn.query_opt("SELECT 1", &[])
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
