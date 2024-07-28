use anyhow::Result;
// use axum::{extract::Path, routing::get, Router};
// use percent_encoding::percent_decode_str;
// use reqwest::StatusCode;
// use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let addr = "0.0.0.0:8080";
    // let listener = TcpListener::bind(&addr).await?;
    info!("listening on {}", addr);

    // let app = Router::new().route("/image/:spec/:url", get(generate));

    // axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

// 解析参数
// async fn generate(Path(Params { spec: url }): Path<Params>) -> Result<String, StatusCode> {
//     let url = percent_decode_str(input).decode_utf_lossy();
// }
