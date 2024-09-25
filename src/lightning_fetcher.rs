// This module has the functionality of ingest data from lightning network for specified env url

use std::{env, sync::LazyLock};

use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};

static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| Client::new());

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LightningResponse {
    pub public_key: String,
    pub alias: String,
    pub channels: i64,
    pub capacity: i64,
    pub first_seen: i64,
    pub updated_at: i64,
    pub city: Option<City>,
    pub country: Option<Country>,
    #[serde(rename = "iso_code")]
    pub iso_code: Option<String>,
    pub subdivision: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct City {
    pub de: Option<String>,
    pub en: String,
    pub es: Option<String>,
    pub fr: Option<String>,
    pub ja: Option<String>,
    #[serde(rename = "pt-BR")]
    pub pt_br: Option<String>,
    pub ru: Option<String>,
    #[serde(rename = "zh-CN")]
    pub zh_cn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub de: String,
    pub en: String,
    pub es: String,
    pub fr: String,
    pub ja: String,
    #[serde(rename = "pt-BR")]
    pub pt_br: String,
    pub ru: String,
    #[serde(rename = "zh-CN")]
    pub zh_cn: String,
}

async fn fetch_data() -> Result<Vec<LightningResponse>, String> {
    let url = env::var("LIGHTNING_URL").expect("LIGHTNING_URL env var is not present");
    let response = HTTP_CLIENT
    .request(Method::GET, &url)
    .send()
    .await;

    // Validate if the response is ok
    if response.is_err() {
        let err = response.unwrap_err();
        return Err(format!("Error: {} when requesting: {}", err.status().unwrap().as_str(), url));
    }

    let response = response.unwrap();
    // Try to deserialize the response, this scenario will matchs only simple schema
    // but it could validate response size or log some additional rich data
    match response.json::<Vec<LightningResponse>>().await {
        Ok(result) => Ok(result),
        Err(e) => {
            Err(format!("The response from {} is not valid as expected to defined schema.", url))
        },
    }
}