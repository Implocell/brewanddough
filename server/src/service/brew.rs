use anyhow::Error;
use sqlx::{pool::PoolConnection, types::Uuid, Any, PgPool, Postgres};

use crate::{model::value_object::brew::BrewObject, repository::brew::PostgresBrewRepository};

#[derive(Clone)]
pub struct BrewService {
    pub repository: PostgresBrewRepository,
}

impl BrewService {
    pub fn new(db: PoolConnection<Postgres>) -> Self {
        BrewService {
            repository: PostgresBrewRepository::new(db),
        }
    }

    // probably needs to return a function that return what is returned now
    // so we can call the method but still satisify route function

    pub async fn brews(mut self) -> Result<Vec<BrewObject>, Error> {
        let result = self.repository.get_brews().await;
        match result {
            Ok(r) => Ok(r.into_iter().map(|b| b.as_brew_object()).collect()),
            Err(e) => Err(e),
        }
    }

    pub async fn create_brew(&mut self, create_brew: BrewObject) -> Result<Uuid, Error> {
        let domain_object = create_brew.to_domain();
        let result = self.repository.create_brew(domain_object.to_object()).await;

        result
    }

    pub async fn get_brew(&mut self, brew_id: Uuid) -> Result<BrewObject, Error> {
        let result = self.repository.get_brew(brew_id).await;
        match result {
            Ok(r) => Ok(r.as_brew_object()),
            Err(e) => Err(e),
        }
    }
}
