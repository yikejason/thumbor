mod config;
mod handler;
mod pb;

use std::{num::NonZeroUsize, sync::Arc};

use anyhow::Result;
use axum::{routing::get, Router};

pub use config::AppConfig;
pub use handler::*;
use lru::LruCache;
pub use pb::*;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

pub async fn get_router() -> Result<Router> {
    let app = Router::new().route("/image/:spec/:url", get(generate_handler));

    Ok(set_layer(app))
}

pub fn set_layer(app: Router) -> Router {
    let cache: Cache = Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(1024).unwrap())));
    app.layer(
        ServiceBuilder::new()
            .layer(AddExtensionLayer::new(cache))
            .into_inner(),
    )
}
