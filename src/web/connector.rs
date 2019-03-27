extern crate lazy_static;
use lazy_static::lazy_static;

use super::super::data::POOL;
use log::{debug, error};

lazy_static! {
    pub static ref CONN: Connector = Connector {
        client: reqwest::Client::new(),
        header_auth: "Bot ".to_string() + &POOL.key,
    };
}

pub struct Connector {
    client: reqwest::Client,
    header_auth: String,
}

impl Connector {
    pub fn get(&self, path: &str) -> reqwest::Result<reqwest::Response> {
        let log_text = format!("Connected to address \"{}\"", path);
        let resp = self
            .client
            .get(path)
            .header(reqwest::header::AUTHORIZATION, &self.header_auth[..])
            .send();
        match resp.as_ref() {
            Ok(msg) => {
                debug!("{}{}", log_text, "Succeded");
                debug!("Status is {}", &msg.status());
            }
            Err(e) => {
                error!("{}{}{}", log_text, "Failed: ", &e);
            }
        };
        return resp;
    }
}
