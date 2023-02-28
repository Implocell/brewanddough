use anyhow::Error;
use sqlx::{pool::PoolConnection, types::Uuid, PgPool, Postgres};

use crate::{
    model::active_record::ingredient::Ingredient,
    repository::ingredient::PostgresIngredientRepository,
};

pub struct IngredientService {
    pub repository: PostgresIngredientRepository,
}

impl IngredientService {
    pub fn new(db: PoolConnection<Postgres>) -> Self {
        IngredientService {
            repository: PostgresIngredientRepository::new(db),
        }
    }

    pub async fn create_ingredient(&mut self, ingredient: Ingredient) -> Result<Uuid, Error> {
        let result = self.repository.create_ingredient(ingredient).await;

        result
    }
}
