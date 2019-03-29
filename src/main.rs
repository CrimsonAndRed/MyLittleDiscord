extern crate log4rs;
extern crate serde;
extern crate serde_json;

extern crate actix;
extern crate actix_web;
extern crate futures;

use log::{debug, error, info};
use actix_web::ws::{Client, ClientWriter, Message, ProtocolError};
use actix::*;
use crate::connector::MyLittleConnection;
use futures::Future;

mod data;
mod discord;
mod engine;
mod connector;

fn main() -> Result<(), Box<std::error::Error>> {
    let sys = actix::System::new("my-little-discord");

    log4rs::init_file("conf/log4rs.yaml", Default::default())?;

    debug!("Starting arbiter");

    Arbiter::spawn( {
        debug!("Trying to create pool");
        let p: &data::Pool = &data::POOL;
        debug!("Done with pool");
        //
        //    let mut resp = web::connector::CONN.get("https://discordapp.com/api/v6/users/@me/guild")?;
        //
        //    info!("{}", &resp.text()?);

        debug!("Connecting to {}", &p.wss_ref);

        Client::new(&p.wss_ref)
            .connect()
            .map_err(|e| {
                error!("Something bad happened: {}", e);
                ()
            })
            .map(|(reader, writer)| {
                let addr = MyLittleConnection::create(|ctx| {
                    MyLittleConnection::add_stream(reader, ctx);
                    MyLittleConnection { writer, last_sequence: None }
                });

                ()
            })
        }
    );

    let _ = sys.run();
    Ok(())
}
