use lazy_static::lazy_static;
use log::{debug, error, info};
use actix_web::client::{ClientResponse, SendRequestError};
use actix_web::error::JsonPayloadError;
use actix_web::client;
use futures::Future;
use actix_web::HttpMessage;


const WSS_ADDRESS_LINK: &'static str = "https://discordapp.com/api/v6/gateway";
const EXPECTED_ARGUMENTS: usize = 2;


lazy_static! {
    pub static ref POOL: Pool = Pool::new();
}

#[derive(Debug)]
pub struct Pool {
    pub wss_ref: String,
    pub key: String,
}

impl Pool {
    fn new() -> Self {
        let wss_ref = Pool::get_wss_ref();
        let key = Pool::get_secure_key();
        Pool {
            key,
            wss_ref,
        }
    }

    fn get_wss_ref() -> String {
        let log_text: String = format!("Tried to connect to address: {}", WSS_ADDRESS_LINK);
        debug!("Getting wss");

        let a = client::get(WSS_ADDRESS_LINK)
                .finish()
                .unwrap();

        debug!("got a");
//        a
//            .send()
//            .map_err(|_| ())
//            .and_then(|x| {
//                debug!("Got \n{:?}", x);
//                Ok(())
//            }).poll();

        debug!("Got b");

//            let c: serde_json::value::Value = b
//                .unwrap()
//                .json()
//                .from_err()
//                .and_then(|x: serde_json::value::Value| {
//                    x
//                }).responder()
//                .wait();
//
//            debug!("Got c");
//            c

//        debug!("Got \n{:?}", b);
//        };

//        if resp.is_ok() {
//            debug!("{}; Succeded", log_text);
//        } else {
//            let err = resp.as_ref().unwrap_err();
//            error!("{}; Failed: {}", log_text, err);
//            panic!("{}", err);
//        }

//        resp.unwrap()
        //--
//            resp
//            .as_object()
//            .unwrap()
//            .get("url")
//            .unwrap()
//            .as_str()
//            .unwrap()
//            .to_string()
        // .wait() does not work in actix-web ??????
        "wss://gateway.discord.gg/?v=6&encoding=json".to_string()
    }

    fn get_secure_key() -> String {
        let args: Vec<String> = std::env::args().collect();

        if args.len() != EXPECTED_ARGUMENTS {
            panic!(
                "Wrong arguments length. Expected: {}, Got: {}",
                EXPECTED_ARGUMENTS,
                args.len()
            );
        }

        let key = &args[1];
        info!("The key is \"{}\"", key);
        key.to_string()
    }
}
