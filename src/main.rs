extern crate log4rs;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

extern crate actix;
extern crate actix_web;
extern crate futures;

use log::{debug, error, info};
use actix_web::ws::{Client, ClientWriter, Message, ProtocolError};
use actix::*;
use crate::engine::MyLittleConnection;
use futures::Future;

mod data;
mod discord;
mod engine;
mod web;

fn main() -> Result<(), Box<std::error::Error>> {
    log4rs::init_file("conf/log4rs.yaml", Default::default())?;

    let p: &data::Pool = &data::POOL;
//
//    let mut resp = web::connector::CONN.get("https://discordapp.com/api/v6/users/@me/guild")?;
//
//    info!("{}", &resp.text()?);

    debug!("Connecting to {}", &p.wss_ref);

    let sys = actix::System::new("my-little-discord");

    Arbiter::spawn(
        Client::new(&p.wss_ref)
            .connect()
            .map_err(|e| {
                error!("Something bad happened: {}", e);
                ()
            })
            .map(|(reader, writer)| {
                let addr = MyLittleConnection::create(|ctx| {
                    MyLittleConnection::add_stream(reader, ctx);
                    MyLittleConnection{writer, last_sequence: None}
                });

                ()
            }),
    );

    let _ = sys.run();
    Ok(())
}
