use serde::{Deserialize, Serialize};

use crate::model::entity::brew::Brew;

#[derive(Deserialize, Serialize)]
pub struct BrewObject {
    #[serde(default)]
    pub brew_id: String,
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub updated_at: i64,
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

impl BrewObject {
    pub fn new(
        brew_id: String,
        created_at: i64,
        updated_at: i64,
        name: String,
        original_gravity: Option<i32>,
        final_gravity: Option<i32>,
        brew_type: i32,
        dry_hops: bool,
        carbonation: Option<i32>,
        carbonation_ingredient: Option<i32>,
        fermentation_ingredient: Option<i32>,
        abv: Option<i32>,
        date_start: Option<i64>,
        date_end: Option<i64>,
    ) -> Self {
        BrewObject {
            brew_id,
            created_at,
            updated_at,
            name,
            original_gravity,
            final_gravity,
            brew_type,
            dry_hops,
            carbonation,
            carbonation_ingredient,
            fermentation_ingredient,
            abv,
            date_start,
            date_end,
        }
    }

    pub fn to_domain(&self) -> Brew {
        Brew::from_object(self)
    }
}
