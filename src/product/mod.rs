mod back_mareket_uuid_extractor;
mod snapshot;

use back_mareket_uuid_extractor::extract_back_market_uuid_from_url;
use serde_derive::Deserialize;
pub use snapshot::Snapshot;

pub struct Product {
    back_market_uuid: String,
    title: String,
}

#[derive(Deserialize)]
struct ApiResponse {
    title: String,
}

impl Product {
    pub async fn from_url(url: &str) -> Result<Self, String> {
        let back_market_uuid = match extract_back_market_uuid_from_url(url) {
            Ok(uuid) => uuid,
            Err(e) => return Err(e.message().to_string()),
        };

        let api_url = format!(
            "https://www.backmarket.co.uk/bm/product/{}/technical_specifications",
            back_market_uuid
        );

        let client = match reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537")
            .build() {
            Ok(client) => client,
            Err(_) => return Err("Failed to create client".to_string()),
        };

        let response = match client
            .get(&api_url)
            .header("Accept-Language", "en-gb")
            .send()
            .await
        {
            Ok(res) => res,
            Err(_) => return Err("Failed to send request".to_string()),
        };

        let api_response: ApiResponse = match response.json().await {
            Ok(json) => json,
            Err(_) => return Err("Failed to parse JSON".to_string()),
        };

        let title = api_response.title;

        Result::Ok(Product {
            back_market_uuid,
            title,
        })
    }

    pub fn back_market_uuid(&self) -> &str {
        &self.back_market_uuid
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}

pub enum Grade {
    Fair,
    Good,
    Excellent,
}
