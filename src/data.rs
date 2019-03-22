const WSS_ADDRESS_LINK: &'static str = "https://discordapp.com/api/v6/gateway";
const EXPECTED_ARGUMENTS: usize = 2;

use lazy_static::lazy_static;
use log::{debug, error, info};

lazy_static! {
    pub static ref POOL: Pool = Pool::new();
}

#[derive(Debug)]
pub struct Pool {
    pub wss_ref: String,
    pub key: String,
    pub client: reqwest::Client,
}

impl Pool {
    fn new() -> Self {
        let client = reqwest::Client::new();
        let wss_ref = Pool::get_wss_ref(&client);
        let key = Pool::get_secure_key();
        Pool {
            client,
            key,
            wss_ref,
        }
    }

    fn get_wss_ref(client: &reqwest::Client) -> String {
        let log_text: String = format!("Tried to connect to address: {}", WSS_ADDRESS_LINK);

        let resp: Result<serde_json::value::Value, reqwest::Error> = {
            client
                .get(WSS_ADDRESS_LINK)
                .send()
                .and_then(|mut x| x.json())
        };

        if resp.is_ok() {
            debug!("{}; Succeded", log_text);
        } else {
            let err = resp.as_ref().unwrap_err();
            error!("{}; Failed: {}", log_text, err);
            panic!("{}", err);
        }

        resp.unwrap()
            .as_object()
            .unwrap()
            .get("url")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
    }

    fn get_secure_key() -> String {
        let args: Vec<String> = std::env::args().collect();

        if args.len() != EXPECTED_ARGUMENTS {
            panic!(
                "Wrong arguments size. Expected: {}, Got: {}",
                EXPECTED_ARGUMENTS,
                args.len()
            );
        }

        let key = &args[1];
        info!("The key is \"{}\"", key);
        key.to_string()
    }
}
