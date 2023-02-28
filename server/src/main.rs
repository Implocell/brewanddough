use crate::config::Config;
use axum::Extension;
use axum::Router;
use clap::Parser;
use router::{api_routes, frontend_routes};
use service::{brew::BrewService, ingredient::IngredientService};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod api;
mod config;
mod model;
mod repository;
mod router;
mod service;

#[derive(Clone)]
pub struct ApiContext {
    db: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv::dotenv().ok();

    let config = Config::parse();

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .expect("could not connect to database_url");

    sqlx::migrate!().run(&db).await?;

    let brew_service_pool = match db.acquire().await {
        Ok(r) => r,
        Err(e) => return Ok(()),
    };

    let ingredient_service_pool = match db.acquire().await {
        Ok(r) => r,
        Err(e) => return Ok(()),
    };

    let brew_service = BrewService::new(brew_service_pool);

    let ingredient_service = IngredientService::new(ingredient_service_pool);

    let app = Router::new()
        .merge(frontend_routes())
        .nest("/api/v1", api_routes(brew_service, ingredient_service))
        .layer(ServiceBuilder::new().layer(Extension(ApiContext { db: db })))
        .layer(CorsLayer::permissive()) // should be enabled for dev only
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 7000));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
