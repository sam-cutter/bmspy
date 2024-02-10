fn main() {
    println!("Hello, world!");
}

struct Product {
    id: String,
    back_market_uuid: String,
    back_maket_slug: String,
    title: String,
}

struct ProductSnapshot {
    id: String,
    product: Product,
    grade: Grade,
    available: bool,
    price: Option<f64>,
}

enum Grade {
    Fair,
    Good,
    Excellent,
}
