mod back_mareket_uuid_extractor;

use back_mareket_uuid_extractor::extract_back_market_uuid_from_url;

pub struct Product {
    back_market_uuid: String,
    title: String,
}

impl Product {
    pub fn from_url(url: &str) -> Result<Self, String> {
        let back_market_uuid = match extract_back_market_uuid_from_url(url) {
            Ok(uuid) => uuid,
            Err(e) => return Err(e.message().to_string()),
        };

        // TODO: Fetch the product title
        let title = String::new();

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

pub struct Snapshot {
    product: Product,
    grade: Grade,
    price: Option<f32>,
}

impl Snapshot {
    pub fn product(&self) -> &Product {
        &self.product
    }

    pub fn grade(&self) -> &Grade {
        &self.grade
    }

    pub fn price(&self) -> &Option<f32> {
        &self.price
    }
}

pub enum Grade {
    Fair,
    Good,
    Excellent,
}
