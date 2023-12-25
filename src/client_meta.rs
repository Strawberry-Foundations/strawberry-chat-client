pub struct ClientMeta {
    pub username: String,
}

impl ClientMeta {
    pub fn new() -> Self {
        Self {
            username: String::new(),
        }
    }
}
