use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::repository::ingredient::IngredientDatabaseObject;

#[derive(Deserialize, Serialize)]
pub struct Ingredient {
    pub id: String,
    pub name: String,
    pub unit: String,
    pub ingredient_type: String,
}
