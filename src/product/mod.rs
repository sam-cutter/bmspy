pub struct Product {
    pub back_market_uuid: String,
    pub title: String,
}

impl Product {
    pub fn new(url: &str) -> Result<Self, &str> {
        // TODO: Extract the Back Market UUID from the product URL
        let back_market_uuid = String::new();
        // TODO: Fetch the product title
        let title = String::new();

        Result::Ok(Product {
            back_market_uuid,
            title,
        })
    }
}

pub struct Snapshot {
    pub product: Product,
    pub grade: Grade,
    pub availability: Availability,
}

pub enum Availability {
    Available(f64),
    NotAvailable,
}

pub enum Grade {
    Fair,
    Good,
    Excellent,
}
