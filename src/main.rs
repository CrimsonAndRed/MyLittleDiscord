extern crate log4rs;
extern crate serde;
extern crate serde_json;

extern crate actix;
extern crate actix_web;
extern crate futures;

use actix::*;
use actix_web::ws::Client;
use connector::*;
use engine::Engine;
use futures::Future;
use log::{debug, error};

mod connector;
mod data;
mod discord;
mod engine;

fn main() -> Result<(), Box<std::error::Error>> {
    let mut sys = actix::System::new("my-little-discord");

    log4rs::init_file("conf/log4rs.yaml", Default::default())?;

    debug!("Trying to create pool");
    let p: &data::Pool = &data::POOL;
    debug!("Done with pool");

    debug!("Starting files folder check");
    create_files_folder()?;
    debug!("Finished files folder");

    let addr = register_actor(RequestConnector {
        key_header: format!("Bot {}", &p.key),
    });

    // Just for test
    Arbiter::spawn({
        System::current()
            .registry()
            .get::<RequestConnector>()
            .send(RequestMessage {
                method: HttpMethod::GET,
                url: "/users/@me/guilds".to_owned(),
                data: None,
            })
            .map_err(|e| {
                error!("Got error {}", e);
                ()
            })
            .map(|msg| {
                debug!("Guilds future returned: {:?}", msg);
                ()
            })
    });

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
                // Black magic from tutorial
                let addr = WssConnector::create(|ctx| {
                    WssConnector::add_stream(reader, ctx);
                    WssConnector {
                        writer,
                        last_sequence: None,
                        engine: Engine::new(),
                    }
                });

                System::current().registry().set(addr);

                ()
            })
    });

    let _ = sys.run();
    Ok(())
}

/// Manually register SystemService in Registry pool
/// All SystemServices have to implement Default, which is messing with complex actors.
/// So all of actors are created manually and then are registered via this function.
fn register_actor<A: SystemService>(actor: A) -> Addr<A> {
    let addr = actor.start();
    System::current().registry().set(addr.clone());
    addr
}

/// Creates folder for files.
fn create_files_folder() -> std::io::Result<()> {
    let mut dir = std::env::current_dir()?;
    dir.push("files");
    std::fs::create_dir_all(dir)
}
