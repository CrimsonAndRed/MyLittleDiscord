extern crate log4rs;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use log::{debug, info};

mod data;
mod web;

fn main() -> Result<(), Box<std::error::Error>> {
    log4rs::init_file("conf/log4rs.yaml", Default::default())?;

    let p: &data::Pool = &data::POOL;

    let mut resp = web::connector::CONN
        .get("https://discordapp.com/api/v6/users/@me/guild")?;

    info!("{:?}", p);
    info!("{}", &resp.text()?);
    debug!("Status is {}", &resp.status());
    info!("---");
    info!("{:?}", &resp.headers());
    Ok(())
}
