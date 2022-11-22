use serde::{Deserialize, Serialize};

use crate::{AppError, Result};

#[derive(Debug, Serialize)]
struct TranslateQuery {
    q: String,
    target: String,
    source: Option<String>,
    format: String,
    model: String,
    key: String,
}

impl TranslateQuery {
    pub fn new(query: &str, api_key: &str) -> Self {
        TranslateQuery {
            q: query.to_string(),
            target: "en".to_string(),
            format: "text".to_string(),
            source: None,
            model: "base".to_string(),
            key: api_key.to_string(),
        }
    }

    pub fn set_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }

    pub fn set_target(mut self, target: String) -> Self {
        self.target = target;
        self
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TranslateOutputData {
    data: OutputTranslations,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OutputTranslations {
    pub translations: Vec<Translation>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Translation {
    pub translated_text: String,
    pub detected_source_language: Option<String>,
    pub model: String,
}

pub struct GoogleCloudClient {
    pub api_key: String,
    pub http_client: reqwest::Client,
}

impl GoogleCloudClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn translate(
        &self,
        query: &str,
        target: &str,
        source: Option<&str>,
    ) -> Result<Translation> {
        log::debug!("Send query to Google Translate: {:?}", query);

        let mut query = TranslateQuery::new(query, &self.api_key)
            .set_target(target.to_string());
        if let Some(source) = source {
            query = query.set_source(source.to_string());
        }

        log::debug!("Serialize query object into json: {:?}", query);

        let res = self
            .http_client
            .post("https://translation.googleapis.com/language/translate/v2")
            .query(&query)
            .header("content-length", 0)
            .send()
            .await?;

        log::info!("Google translate response status: {:?}", res.status());
        log::debug!("{:?}", res);
        if res.status() != 200 {
            return Err(AppError {
                msg: format!("Google Cloud Translate Error: {}", res.status()),
            });
        }

        let out = res.json::<TranslateOutputData>().await?;

        if out.data.translations.is_empty() {
            return Err(AppError {
                msg: "Bad Response: Translations are missing".to_string(),
            });
        }

        Ok(out.data.translations[0].clone())
    }
}
