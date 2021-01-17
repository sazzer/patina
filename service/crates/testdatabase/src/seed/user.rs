use std::collections::HashMap;

use chrono::{DateTime, Timelike, Utc};
use postgres_types::ToSql;
use serde_json::Value;
use uuid::Uuid;

use super::SeedData;

/// Representation of a user ready to seed into the test database.
#[derive(Debug)]
pub struct SeedUser {
    pub user_id:         Uuid,
    pub version:         Uuid,
    pub created:         DateTime<Utc>,
    pub updated:         DateTime<Utc>,
    pub email:           String,
    pub display_name:    String,
    pub authentications: Value,
}

impl Default for SeedUser {
    fn default() -> Self {
        let now = Utc::now().with_nanosecond(0).unwrap();

        Self {
            user_id:         Uuid::new_v4(),
            version:         Uuid::new_v4(),
            created:         now,
            updated:         now,
            email:           format!("{}", Uuid::new_v4()),
            display_name:    format!("{}", Uuid::new_v4()),
            authentications: Value::Array(vec![]),
        }
    }
}

impl SeedUser {
    /// Add some authentication details to the user
    ///
    /// # Parameters
    /// - `service` - The name of the authentication service
    /// - `id` - The ID of the user at this service
    /// - `display_name` - The display name of the user at this service
    pub fn with_authentication<S, I, D>(mut self, service: S, id: I, display_name: D) -> Self
    where
        S: Into<String>,
        I: Into<String>,
        D: Into<String>,
    {
        let mut authentication = HashMap::new();
        authentication.insert("service", service.into());
        authentication.insert("id", id.into());
        authentication.insert("displayName", display_name.into());

        self.authentications
            .as_array_mut()
            .unwrap()
            .push(serde_json::to_value(authentication).unwrap());

        self
    }
}

impl SeedData for SeedUser {
    fn sql(&self) -> &str {
        "INSERT INTO users(user_id, version, created, updated, email, display_name, authentications)
          VALUES ($1, $2, $3, $4, $5, $6, $7)"
    }

    fn binds(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.user_id,
            &self.version,
            &self.created,
            &self.updated,
            &self.email,
            &self.display_name,
            &self.authentications,
        ]
    }
}
