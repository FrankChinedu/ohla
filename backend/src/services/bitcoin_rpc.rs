pub struct BitcoinRpc {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl BitcoinRpc {
    pub fn new(url: String, username: String, password: String) -> Self {
        Self {
            url,
            username,
            password,
        }
    }
}
