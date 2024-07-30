use std::{
    hash::{DefaultHasher, Hash, Hasher},
    sync::Arc,
};

use anyhow::Result;
use axum::{
    extract::{Extension, Path},
    http::{HeaderMap, HeaderValue, StatusCode},
};
use bytes::Bytes;
use image::ImageOutputFormat;
use lru::LruCache;
use percent_encoding::{percent_decode_str, percent_encode, NON_ALPHANUMERIC};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::{info, instrument};

use crate::{filter, resize, Engine, ImageSpec, Photon, Spec};

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    spec: String,
    url: String,
}

pub type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

// TODO: 稍后理解下
pub async fn generate_handler(
    Path(Params { spec, url }): Path<Params>,
    Extension(cache): Extension<Cache>,
) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Ok(format!("url: {}\n spec: {:#?}", url, spec))
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let data = retri_image(&url, cache)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // TODO: 处理图片
    let mut engine: Photon = data
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    engine.apply(&spec.specs);

    let image = engine.generate(ImageOutputFormat::Jpeg(85));
    info!("Finished processing: image size {}", image.len());

    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", HeaderValue::from_static("image/jpeg"));
    Ok((headers, image.to_vec()))
}

#[instrument(level = "info", skip(cache))]
async fn retri_image(url: &str, cache: Cache) -> Result<Bytes> {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let key = hasher.finish();
    // TODO: 稍后理解下
    let g = &mut cache.lock().await;
    let data = match g.get(&key) {
        Some(v) => {
            info!("Match cache {}", key);
            v.to_owned()
        }
        None => {
            info!("Retrieve url");
            let resp = reqwest::get(url).await?;
            let data = resp.bytes().await?;
            g.put(key, data.clone());
            data
        }
    };

    Ok(data)
}

// 调试辅助函数
pub fn print_test_url(url: &str) {
    use std::borrow::Borrow;
    let spec1 = Spec::new_resize(500, 800, resize::SampleFilter::CatmullRom);
    let spec2 = Spec::new_watermark(20, 20);
    let spec3 = Spec::new_filter(filter::Filter::Marine);
    let image_spec = ImageSpec::new(vec![spec1, spec2, spec3]);
    let s: String = image_spec.borrow().into();
    let test_image = percent_encode(url.as_bytes(), NON_ALPHANUMERIC).to_string();
    println!("test url: http://localhost:3000/image/{}/{}", s, test_image);
}
