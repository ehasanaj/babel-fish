use std::{env, error::Error};

use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    StatusCode,
};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;

static URI: &str = "https://api-free.deepl.com/v2/translate";
static DEFAULT_CONTENT_TYPE: &str = "application/json";
static DEFAULT_ACCEPT: &str = "application/json";

#[derive(Serialize, Deserialize)]
struct Translation {
    detected_source_language: String,
    text: String,
}

#[derive(Serialize, Deserialize)]
struct Response {
    translations: Vec<Translation>,
}

pub async fn execute(target_language: &str, text: &str) -> Result<String, Box<dyn Error>> {
    let api_key = env::var("DEEPL_API_KEY")
        .expect("DEEPL_API_KEY environment variable should be set with your deepl api key!");

    let request = json!({
        "text": [text],
        "target_lang": target_language
    });

    let authorization = format!("DeepL-Auth-Key {api_key}");

    let response = reqwest::Client::new()
        .get(URI)
        .header(CONTENT_TYPE, DEFAULT_CONTENT_TYPE)
        .header(ACCEPT, DEFAULT_ACCEPT)
        .header(AUTHORIZATION, authorization)
        .body(request.to_string())
        .send()
        .await?;

    if response.status() != StatusCode::OK {
        return Err(format!("Translation failed with error code {}", response.status()).into());
    }

    let response_bytes = response.bytes().await?;
    let response_obj: Response = serde_json::from_slice(&response_bytes)?;
    let response_text = &response_obj
        .translations
        .get(0)
        .expect("Translation not found")
        .text;

    Ok(response_text.to_string())
}
