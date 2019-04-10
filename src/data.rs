use lazy_static::lazy_static;
use log::info;

const WSS_ADDRESS_LINK: &'static str = "https://discordapp.com/api/v6/gateway";
const EXPECTED_ARGUMENTS: usize = 2;

lazy_static! {
    pub static ref POOL: Pool = Pool::new();
}

/// Pool of static data.
#[derive(Debug)]
pub struct Pool {
    pub wss_ref: String,
    pub key: String,
}

impl Pool {
    fn new() -> Self {
        let wss_ref = Pool::get_wss_ref();
        let key = Pool::get_secure_key();
        Pool { key, wss_ref }
    }

    fn get_wss_ref() -> String {
        // It is an option to ask DISCORD for wss address
        // But it creates cyclic dependencies between POOL, RequestConnector and ActorsPool
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
