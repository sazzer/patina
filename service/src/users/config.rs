use std::sync::Arc;

use actix_web::web::ServiceConfig;

use super::{
    endpoints::configure_server, repository::Repository, service::UsersService, GetUserUseCase,
};
use crate::{database::Database, server::Configurer};

/// Configuration component for working with users.
pub struct Component {
    service: Arc<UsersService>,
}

/// Construct a new users component.
pub fn new(database: Arc<Database>) -> Arc<Component> {
    tracing::debug!("Building users service");
    let repository = Repository::new(database);
    let service = UsersService::new(repository);

    Arc::new(Component {
        service: Arc::new(service),
    })
}

impl Configurer for Component {
    fn configure_server(&self, config: &mut ServiceConfig) {
        config.data(self.service.clone() as Arc<dyn GetUserUseCase>);

        configure_server(config);
    }
}
