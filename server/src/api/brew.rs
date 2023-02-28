use axum::Extension;
use axum::{http::StatusCode, Json};
use sqlx::types::Uuid;

use crate::model::value_object::brew::BrewObject;
use crate::service::brew::BrewService;

pub async fn brews(
    Extension(brew_service): Extension<BrewService>,
) -> (StatusCode, Json<Vec<BrewObject>>) {
    let result = brew_service.brews().await;
    match result {
        Ok(brews) => (StatusCode::OK, Json(brews)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
    }
}

pub async fn create_brew(
    Extension(mut brew_service): Extension<BrewService>,
    Json(payload): Json<BrewObject>,
) -> (StatusCode, Json<String>) {
    let result = brew_service.create_brew(payload).await;

    match result {
        Ok(id) => (StatusCode::CREATED, Json(id.to_string())),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())),
    }
}
pub async fn get_brew(
    Extension(mut brew_service): Extension<BrewService>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> (StatusCode, Json<BrewObject>) {
    let result = match Uuid::parse_str(&id) {
        Ok(u) => brew_service.get_brew(u).await,
        Err(e) => Err(anyhow::format_err!(e.to_string())),
    };

    match result {
        Ok(b) => (StatusCode::OK, Json(b)),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(BrewObject {
                brew_id: String::from(""),
                created_at: 0,
                updated_at: 0,
                name: String::from(""),
                abv: None,
                brew_type: -1,
                original_gravity: None,
                carbonation: None,
                carbonation_ingredient: None,
                date_end: None,
                date_start: None,
                dry_hops: false,
                fermentation_ingredient: None,
                final_gravity: None,
            }),
        ),
    }
}
