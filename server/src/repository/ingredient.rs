use anyhow::Context;
use sqlx::pool::PoolConnection;
use sqlx::types::Uuid;
use sqlx::{PgPool, Postgres};

use crate::model::active_record::ingredient::Ingredient;

pub struct IngredientDatabaseObject {
    pub id: Uuid,
    pub name: String,
    pub unit: String,
    pub ingredient_type: String,
}

pub struct PostgresIngredientRepository {
    db: PoolConnection<Postgres>,
}

impl PostgresIngredientRepository {
    pub fn new(db: PoolConnection<Postgres>) -> Self {
        Self { db }
    }

    pub async fn create_ingredient(&mut self, ingredient: Ingredient) -> anyhow::Result<Uuid> {
        sqlx::query_scalar!(
            r#"insert into "ingredient" (
            name,
            unit,
            ingredient_type
        ) values ($1, $2, $3) returning ingredient_id"#,
            ingredient.name,
            ingredient.unit,
            ingredient.ingredient_type
        )
        .fetch_one(&mut self.db)
        .await
        .context("could not save brew")
    }
}
