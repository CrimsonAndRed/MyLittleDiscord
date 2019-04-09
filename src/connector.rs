use log::{debug, info, error, warn};

use crate::discord::MessagePacket;
use crate::discord::OpCode;
use crate::data::POOL;
use crate::discord::IdentityPacket;
use crate::discord::IdentityPropertiesPacket;
use actix_web::ws::{Client, ClientWriter, Message, ProtocolError};
use actix_web::client;
use actix::*;
use actix_web::client::{ClientResponse, SendRequestError};
use futures::Future;
use actix_web::HttpMessage;
use actix_web::actix::dev::MessageResponse;

/// Internal engine that handles DISCORD messages.
pub struct MyLittleConnection {

    pub writer: ClientWriter,

    pub last_sequence: Option<i64>,
}

impl Actor for MyLittleConnection {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        debug!("Started MyLittleConnection");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
        debug!("Stopped MyLittleConnection");
    }
}

impl StreamHandler<Message, ProtocolError> for MyLittleConnection {
    fn handle(&mut self, msg: Message, ctx: &mut Context<Self>) {
        match msg {
            Message::Text(txt) => info!("Got msg\n{:?}", txt),
            Message::Close(e) => {
                warn!("Received close message from DISCORD, with reason {:?}", &e);
                warn!("Closing connection and exitting");
                // self.writer.close(e);
                // Do we need to wait??
                System::current().stop();
            },
            Message::Ping(d) => {
                info!("Received ping message from DISCORD, with text {}", &d);
                info!("Responding with pong with same text");
                self.writer.pong(&d);
            },
            Message::Binary(_) => {
                info!("Received binary message from DISCORD. Skipping.");
            },
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

impl Actor for RequestConnector {
    type Context = Context<Self>;
}

/// Async
impl Handler<RequestMessage> for RequestConnector {
    type Result = Box<dyn Future<Item = serde_json::Value, Error = actix_web::error::Error>>;

    fn handle(&mut self, msg: RequestMessage, ctx: &mut Context<Self>) -> Self::Result {

        debug!("Handled msg {:?}", msg);
        let res = client::get(msg.url)
            .header(actix_web::http::header::AUTHORIZATION, self.key_header.to_string())
            .finish()
            .unwrap()
            .send();
        debug!("Prepared res");
        let res = res
            .map_err(actix_web::error::Error::from)
            .and_then(|resp| {
                resp.json()
                    .from_err()
                    .then(|item| item)
            });
        debug!("Got res");

        Box::new(res)
    }
}

#[derive(Debug)]
pub struct RequestMessage {
    pub method: HttpMethod,
    pub url: String,
    pub data: Option<String>, //??? TODO
}

impl actix::Message for RequestMessage {
    type Result = std::result::Result<serde_json::Value, actix_web::Error>;
}

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE
}