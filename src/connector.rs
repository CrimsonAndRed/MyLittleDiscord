use log::{debug, error, info, warn};

use crate::discord::WrapperPacket;
use crate::engine::*;
use actix::*;
use actix_web::client;
use actix_web::ws::{ClientWriter, Message, ProtocolError};
use actix_web::HttpMessage;
use futures::Future;

/// Internal engine that handles DISCORD messages.
pub struct WssConnector {
    /// Pushes my messages to websocket's destination
    pub writer: ClientWriter,
    /// Last sequence number from DISCORD
    pub last_sequence: Option<i64>,
    /// Engine
    pub engine: Engine,
}

/// Dont look here ~
impl Default for WssConnector {
    fn default() -> Self {
        unimplemented!("This should never happen. All actors are started manually")
    }
}

impl Actor for WssConnector {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        debug!("Started MyLittleConnection");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        debug!("Stopped MyLittleConnection");
    }
}

// Wow i spent so much time figuring out how to put addresses to special pool
// But it seems like already implemented feature in actix
// I have to play with Default though, maybe there is a way to avoid implementing Default when i construct all my actors manually?
// I also like that official user guide ignores registries.
impl actix::Supervised for WssConnector {}
impl SystemService for WssConnector {}

impl StreamHandler<Message, ProtocolError> for WssConnector {
    fn handle(&mut self, msg: Message, ctx: &mut Context<Self>) {
        match msg {
            Message::Text(txt) => {
                info!("Got msg {:?}", txt);
                let json: serde_json::Result<WrapperPacket> = serde_json::from_str(&txt);
                match json {
                    Err(e) => {
                        error!("Failed to parse packet as json {}. Ignoring packet.", e);
                    }
                    Ok(content) => {
                        self.last_sequence = content.s;
                        self.engine.on_message(content);
                    }
                }
            }
            Message::Close(e) => {
                warn!("Received close message from DISCORD, with reason {:?}", &e);
                warn!("Closing connection and exitting");
                // self.writer.close(e);
                // Do we need to wait??
                System::current().stop();
            }
            Message::Ping(d) => {
                info!("Received ping message from DISCORD, with text {}", &d);
                info!("Responding with pong with same text");
                self.writer.pong(&d);
            }
            Message::Binary(_) => {
                info!("Received binary message from DISCORD. Skipping.");
            }
            Message::Pong(_) => {
                info!("Received pong message from DISCORD. Skipping.");
            }
        }
    }

    fn started(&mut self, ctx: &mut Context<Self>) {
        debug!("Connected");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        debug!("Finished");
    }
}

pub struct RequestConnector {
    pub key_header: String,
}

/// Dont look here ~
impl Default for RequestConnector {
    fn default() -> Self {
        unimplemented!("This should never happen. All actors are started manually")
    }
}

impl Actor for RequestConnector {
    type Context = Context<Self>;
}

impl actix::Supervised for RequestConnector {}
impl SystemService for RequestConnector {}

impl Handler<RequestMessage> for RequestConnector {
    type Result = Box<dyn Future<Item = serde_json::Value, Error = actix_web::error::Error>>;

    fn handle(&mut self, msg: RequestMessage, ctx: &mut Context<Self>) -> Self::Result {
        let url = &msg.url;
        let mut req = match &msg.method {
            HttpMethod::GET => client::get(url),
            HttpMethod::POST => client::post(url),
            HttpMethod::PUT => client::put(url),
            HttpMethod::DELETE => client::delete(url),
        };
        let req = req.header(
            actix_web::http::header::AUTHORIZATION,
            self.key_header.to_string(),
        );

        let req = if msg.data.is_some() {
            req.json(&msg.data)
        } else {
            req.finish()
        };

        let res = req.unwrap().send();

        debug!("Handled msg {:?}", &msg);
        let res = res.map_err(actix_web::error::Error::from).and_then(|resp| {
            resp.json()
                .from_err()
                // How do i do it better?
                .then(|item| item)
        });

        Box::new(res)
    }
}

/// Message to get some information from DISCORD REST API.
#[derive(Debug)]
pub struct RequestMessage {
    pub method: HttpMethod,
    pub url: String,
    pub data: Option<serde_json::Value>,
}

impl actix::Message for RequestMessage {
    // WHY??
    type Result = std::result::Result<serde_json::Value, actix_web::Error>;
}

/// Message to response to DISCORD gateway through websockets.
pub struct ClientMessage {
    pub data: WrapperPacket,
}

impl actix::Message for ClientMessage {
    type Result = Result<(), actix_web::error::Error>;
}

impl Handler<ClientMessage> for WssConnector {
    type Result = Result<(), actix_web::error::Error>;

    fn handle(&mut self, mut msg: ClientMessage, ctx: &mut Context<Self>) -> Self::Result {
        debug!("Sending client message to DISCORD: {:?}", msg.data);
        std::mem::replace(&mut msg.data.s, self.last_sequence);
        let json = serde_json::to_string(&msg.data);
        match json {
            Ok(json) => {
                self.writer.text(json);
                // TODO acknowledge that message was received?
                Ok(())
            }
            Err(e) => {
                error!("Could not serialize json with error:\n{}", e);
                Err(actix_web::error::Error::from(e))
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}
