use crate::app::command::generate_url::ShortUrlProvider;

pub struct NanoUrlShortener;

impl ShortUrlProvider for NanoUrlShortener {
    fn provide(&self) -> String {
        nanoid::nanoid!(10)
    }
}
pub struct FakeUrlShortener {
    id: String,
}

impl FakeUrlShortener {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn set_id(&mut self, new_id: String) {
        self.id = new_id;
    }
}

impl ShortUrlProvider for FakeUrlShortener {
    fn provide(&self) -> String {
        self.id.clone()
    }
}
