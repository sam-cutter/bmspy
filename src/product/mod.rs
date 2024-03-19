mod back_mareket_uuid_extractor;
mod title_fetcher;

use back_mareket_uuid_extractor::extract_back_market_uuid_from_url;
use title_fetcher::fetch_title;

pub struct Product {
    back_market_uuid: String,
    title: String,
}

impl Product {
    pub async fn from_url(url: &str) -> Result<Self, String> {
        let back_market_uuid = match extract_back_market_uuid_from_url(url) {
            Ok(uuid) => uuid,
            Err(e) => return Err(e.message().to_string()),
        };

        let title = match fetch_title(back_market_uuid.as_str()).await {
            Ok(title) => title,
            Err(e) => return Err(e.message().to_string()),
        };

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

    pub async fn track(&self, pool: &sqlx::PgPool) {
        let back_market_uuid = self.back_market_uuid();
        let title = self.title();

        let rows_affected = sqlx::query!(
            r#"
            INSERT INTO products (back_market_uuid, title)
            VALUES ($1, $2)
            "#,
            back_market_uuid,
            title
        )
        .execute(pool)
        .await;
    }
}
