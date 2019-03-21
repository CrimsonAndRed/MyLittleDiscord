extern crate reqwest;
extern crate log4rs;

use log::{info, debug};

const EXPECTED_ARGUMENTS: usize = 2;

fn main() -> Result<(), Box<std::error::Error>> {

    log4rs::init_file("conf/log4rs.yaml", Default::default())?;

    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != EXPECTED_ARGUMENTS {
        panic!("Wrong arguments size. Expected: {}, Got: {}", EXPECTED_ARGUMENTS, args.len());
    }

    let key = &args[1];

    info!("The key is \"{}\"", key);

    let client = reqwest::Client::new();

    let mut resp = client.get("https://discordapp.com/api/users/@me/guilds")
        .header("Authorization", "Bot ".to_string() + &key)
        .send()?;
    info!("{}", &resp.text()?);
    debug!("Status is {}", &resp.status());
    info!("---");
    info!("{:?}", &resp.headers());
    Ok(())
}