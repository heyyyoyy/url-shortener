use crate::app::command::generate_url::ShortUrlProvider;

pub struct NanoShortUrl;

impl ShortUrlProvider for NanoShortUrl {
    fn provide(&self) -> String {
        nanoid::nanoid!(10)
    }
}
pub struct FakeShortUrl {
    id: String,
}

impl FakeShortUrl {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn set_id(&mut self, new_id: String) {
        self.id = new_id;
    }
}

impl ShortUrlProvider for FakeShortUrl {
    fn provide(&self) -> String {
        self.id.clone()
    }
}
