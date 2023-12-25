pub struct ClientMeta {
    pub username: String,
}

impl ClientMeta {
    pub fn new() -> ClientMeta {
        ClientMeta {
            username: String::new(),
        }
    }
}