extern crate lazy_static;
use lazy_static::lazy_static;

use super::super::data::POOL;

lazy_static! {
    pub static ref CONN: Connector = Connector {
        client: reqwest::Client::new()
    };
}

pub struct Connector {
    client: reqwest::Client,
}

impl Connector {
    pub fn get(&self, path: &str) -> reqwest::Result<reqwest::Response> {
        self.client
            .get(path)
            .header("Authorization", "Bot ".to_string() + &POOL.key)
            .send()
    }
}
