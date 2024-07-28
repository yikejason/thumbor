mod config;
mod handler;
mod pb;

use anyhow::Result;
use axum::{routing::get, Router};

pub use config::AppConfig;
pub use handler::*;
pub use pb::*;

pub async fn get_router() -> Result<Router> {
    let app = Router::new().route("/image/:spec/:url", get(param_handler));

    Ok(app)
}
