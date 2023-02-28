use axum::Extension;
use axum::{http::StatusCode, Json};

use crate::model::active_record::ingredient::Ingredient;
use crate::service::ingredient::IngredientService;

pub async fn create_ingredient(
    Extension(ingredient_service): Extension<IngredientService>,
    Json(payload): Json<Ingredient>,
) -> (StatusCode, Json<String>) {
    let result = ingredient_service.create_ingredient(payload).await;

    match result {
        Ok(id) => (StatusCode::CREATED, Json(id.to_string())),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())),
    }
}
