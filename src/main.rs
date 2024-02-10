fn main() {
    println!("Hello, world!");
}

struct Product {
    id: String,
    back_market_uuid: String,
    back_maket_slug: String,
    title: String,
}

struct Snapshot {
    id: String,
    product: Product,
    grade: Grade,
    availability: Availability,
}

enum Availability {
    Available(f64),
    NotAvailable,
}

enum Grade {
    Fair,
    Good,
    Excellent,
}
