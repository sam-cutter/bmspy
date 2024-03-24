mod from_url;
mod tracked;

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
}
