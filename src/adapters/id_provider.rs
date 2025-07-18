pub trait IdProvider {
    fn provide(&self) -> String;
}

pub struct NanoIdProvider;

impl IdProvider for NanoIdProvider {
    fn provide(&self) -> String {
        nanoid::nanoid!(10)
    }
}
pub struct FakeIdProvider {
    id: String,
}

impl FakeIdProvider {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn set_id(&mut self, new_id: String) {
        self.id = new_id;
    }
}

impl IdProvider for FakeIdProvider {
    fn provide(&self) -> String {
        self.id.clone()
    }
}
