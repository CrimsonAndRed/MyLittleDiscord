extern crate log4rs;
extern crate serde;
extern crate serde_json;

extern crate actix;
extern crate actix_web;
extern crate futures;

use actix::*;
use actix_web::ws::Client;
use futures::Future;
use log::{debug, error};
use connector::*;

mod connector;
mod data;
mod discord;
mod engine;

fn main() -> Result<(), Box<std::error::Error>> {
    let mut sys = actix::System::new("my-little-discord");

    log4rs::init_file("conf/log4rs.yaml", Default::default())?;

    debug!("Trying to create pools");
    let p: &data::Pool = &data::POOL;

    let addr = register_actor(RequestConnector {
        key_header: format!("Bot {}", &p.key),
    });

    debug!("Done with pools");

    // Just for test
    let guilds_future = System::current().registry().get::<RequestConnector>().send(RequestMessage {
        method: HttpMethod::GET,
        url: "https://discordapp.com/api/v6/users/@me/guilds".to_owned(),
        data: None,
    });
    let res = sys.block_on(guilds_future).unwrap();
    debug!("Guilds future returned: {:?}", res);

    debug!("Starting arbiter");
    Arbiter::spawn({
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
                    MyLittleConnection {
                        writer,
                        last_sequence: None,
                    }
                });

                System::current().registry().set(addr);

                ()
            })
    });

    let _ = sys.run();
    Ok(())
}

fn register_actor<A: SystemService>(actor: A) -> Addr<A> {
    let addr = actor.start();
    System::current().registry().set(addr.clone());
    addr
}
