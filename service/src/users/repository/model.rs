use serde::Deserialize;
use serde_json::{from_value, Value};
use tokio_postgres::Row;

use crate::{
    model::Identity,
    users::{Authentication, AuthenticationId, AuthenticationService, UserData, UserResource},
};

/// A set of authentication details for the user.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationDetail {
    pub service:      AuthenticationService,
    pub id:           AuthenticationId,
    pub display_name: String,
}

impl From<Row> for UserResource {
    fn from(row: Row) -> Self {
        let authentications: Value = row.get("authentications");
        let authentication_details: Vec<AuthenticationDetail> =
            from_value(authentications).expect("Failed to parse authentication details");

        Self {
            identity: Identity {
                id:      row.get("user_id"),
                version: row.get("version"),
                created: row.get("created"),
                updated: row.get("updated"),
            },
            data:     UserData {
                display_name:           row.get("display_name"),
                email:                  row.get("email"),
                authentication_details: authentication_details
                    .into_iter()
                    .map(|a| Authentication {
                        service:      a.service,
                        id:           a.id,
                        display_name: a.display_name,
                    })
                    .collect(),
            },
        }
    }
}
