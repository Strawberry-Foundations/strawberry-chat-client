pub struct ClientMeta {
    pub username: String,
}

impl Default for ClientMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientMeta {
    pub fn new() -> Self {
        Self {
            username: String::new(),
        }
    }
}
