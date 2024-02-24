use super::Product;
pub enum Grade {
    Fair,
    Good,
    Excellent,
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
