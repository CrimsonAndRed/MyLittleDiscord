extern crate log4rs;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

extern crate futures;
extern crate tokio;
extern crate websocket;

use futures::future::Future;
use futures::stream::Stream;
use log::{debug, error, info};
use websocket::{ClientBuilder, OwnedMessage};

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

    let mut runtime = tokio::runtime::current_thread::Builder::new()
        .build()
        .unwrap();
    debug!("Connecting to {}", &p.wss_ref);

    let runner = ClientBuilder::new(&p.wss_ref)
        .unwrap()
        .async_connect_secure(None)
        .and_then(|(duplex, _)| {
            let (sink, stream) = duplex.split();

			let mut server = engine::MyLittleServer {
//				sink: &sink,
				last_sequence: None
			};

            // Might be better to use something like Actix here and register sender and receiver actors
            stream
                .filter_map(move |message| {
                    server.on_discord_message(message)
                })
                .forward(sink)
        });
    runtime.block_on(runner).unwrap();
    Ok(())
}
