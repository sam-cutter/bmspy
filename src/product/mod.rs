pub struct Product {
    pub id: String,
    pub back_market_uuid: String,
    pub title: String,
}

pub struct Snapshot {
    pub id: String,
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
