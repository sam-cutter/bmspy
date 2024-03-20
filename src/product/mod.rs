mod from_url;

pub struct Product {
    uuid: String,
    title: String,
}

impl Product {
    pub fn uuid(&self) -> &str {
        &self.uuid
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub async fn track(&self, pool: &sqlx::PgPool) {
        let uuid = self.uuid();
        let title = self.title();
    }
}
