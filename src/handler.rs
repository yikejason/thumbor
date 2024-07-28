use axum::extract::Path;
use percent_encoding::percent_decode_str;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::ImageSpec;

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    spec: String,
    url: String,
}

pub async fn param_handler(Path(Params { spec, url }): Path<Params>) -> Result<String, StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(format!("url: {}\n spec: {:#?}", url, spec))
}
