use anyhow::Context;
use sqlx::Any;
use sqlx::query_as;
use sqlx::PgPool;
use sqlx::types::Uuid;
use sqlx::types::time::OffsetDateTime;
use sqlx::{pool::PoolConnection, Postgres};
use crate::model::value_object::brew::BrewObject;


pub struct BrewDatabaseObject {
    pub brew_id: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub name: String,
    pub original_gravity: Option<i32>,
    pub final_gravity: Option<i32>,
    pub brew_type: i32,
    pub dry_hops: bool,
    pub carbonation: Option<i32>,
    pub carbonation_ingredient: Option<i32>,
    pub fermentation_ingredient: Option<i32>,
    pub abv: Option<i32>,
    pub date_start: Option<i64>,
    pub date_end: Option<i64>,
}

impl BrewDatabaseObject {
    pub fn as_brew_object(&self) -> BrewObject {
        BrewObject::new(
            self.brew_id.to_string(),
            self.created_at.unix_timestamp(),
            self.updated_at.unix_timestamp(),
            self.name.to_string(),
            self.original_gravity,
            self.final_gravity,
            self.brew_type,
            self.dry_hops,
            self.carbonation,
            self.carbonation_ingredient,
            self.fermentation_ingredient,
            self.abv,
            self.date_start,
            self.date_end,
        )
    }
}


#[derive(Clone)]
pub struct PostgresBrewRepository {
    db: PoolConnection<Postgres>,
}

impl PostgresBrewRepository {
    pub fn new( db: PoolConnection<Postgres>) -> Self {
        Self { db }
    }

    pub async fn get_brews(&mut self) -> anyhow::Result<Vec<BrewDatabaseObject>> {
        query_as!(BrewDatabaseObject, r#"select * from "brew""#)
            .fetch_all(&mut self.db)
            .await
            .context("an unexpected error happened when getting brews")
    }

    pub async fn get_brew(&mut self, id: Uuid) -> anyhow::Result<BrewDatabaseObject> {
        query_as!(BrewDatabaseObject, "select * from brew where brew_id = $1", id).fetch_one(&mut self.db).await.context("unexpected error when getting brew")
    }


pub async fn create_brew(
    &mut self,
    brew: BrewObject,
) -> anyhow::Result<Uuid> {

      sqlx::query_scalar!(
        r#"insert into "brew" (
            name,
            original_gravity,
            final_gravity ,
            abv,
            date_start,
            date_end,
            brew_type,
            dry_hops,
            carbonation,
            carbonation_ingredient,
            fermentation_ingredient) values ($1, $2, $3, $4, $5, $6, $7, $8,$9,$10,$11) returning brew_id"#,
        brew.name, 
        brew.original_gravity,
        brew.final_gravity,
        brew.abv,
        brew.date_start,
        brew.date_end,
        brew.brew_type,
        brew.dry_hops,
        brew.carbonation,
        brew.carbonation_ingredient,
        brew.fermentation_ingredient
    )
    .fetch_one(&mut self.db).await.context("could not save brew")

    
}

}
