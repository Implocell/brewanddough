use axum::{
    body::{boxed, Full},
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{get, post, Router},
    Extension,
};
use rust_embed::RustEmbed;

use crate::{
    api,
    service::{brew::BrewService, ingredient::IngredientService},
};

pub fn frontend_routes() -> Router {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/index.html", get(index_handler))
        .route("/assets/*file", get(static_handler));

    return app;
}

pub fn api_routes(brew_service: BrewService, ingredient_service: IngredientService) -> Router {
    Router::new()
        .route("/brews", get(api::brew::brews))
        // .route("/brews/create", post(api::brew::create_brew))
        // .route("/brews/:id", get(api::brew::get_brew))
        .layer(Extension(brew_service))
    // .route(
    //     "/ingredient/create",
    //     post(api::ingredient::create_ingredient),
    // )
    // .layer(Extension(ingredient_service))
}

async fn index_handler() -> impl IntoResponse {
    static_handler("/index.html".parse::<Uri>().unwrap()).await
}

// We use a wildcard matcher ("/dist/*file") to match against everything
// within our defined assets directory. This is the directory on our Asset
// struct below, where folder = "examples/public/".
async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.starts_with("dist/") {
        path = path.replace("dist/", "");
    }

    StaticFile(path)
}

#[derive(RustEmbed)]
#[folder = "../client/dist"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let body = boxed(Full::from(content.data));
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(body)
                    .unwrap()
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(boxed(Full::from("404")))
                .unwrap(),
        }
    }
}
