use log::{debug, info, error};
//use websocket::OwnedMessage;
//use futures::stream::SplitSink;
//use websocket::client::r#async::{Framed, TlsStream, TcpStream};
//use websocket::codec::ws::MessageCodec;
//use futures::Sink;

use crate::discord::MessagePacket;
use crate::discord::OpCode;
use crate::data::POOL;
use crate::discord::IdentityPacket;
use crate::discord::IdentityPropertiesPacket;
use actix_web::ws::{Client, ClientWriter, Message, ProtocolError};
use actix::*;

/// Internal engine that handles DISCORD messages.
pub struct MyLittleConnection {

    pub writer: ClientWriter,

    pub last_sequence: Option<i64>,
}

#[derive(Message)]
pub struct ClientCommand(String);

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
            _ => (),
        }
    }

    fn started(&mut self, ctx: &mut Context<Self>) {
        debug!("Connected");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        debug!("Finished");
    }
}

impl Handler<ClientCommand> for MyLittleConnection {
    type Result = ();

    fn handle(&mut self, msg: ClientCommand, ctx: &mut Context<Self>) {
        debug!("Got message\n{:?}", msg.0);
    }
}

//impl MyLittleConnection {
//
//    pub fn on_discord_message(&mut self, message: OwnedMessage) -> Option<OwnedMessage> {
//
//        match message {
//            OwnedMessage::Close(e) => {
//                info!("Received close message from DISCORD");
//                Some(OwnedMessage::Close(e))
//            }
//            OwnedMessage::Ping(d) => {
//                info!("Received pong message from DISCORD");
//                Some(OwnedMessage::Pong(d))
//            }
//            OwnedMessage::Text(text) => {
//                info!("Received text message from DISCORD");
//                self.on_text_message(text)
//            }
//            OwnedMessage::Binary(_) => {
//                info!("Received binary message from DISCORD. Skipping.");
//                None
//            }
//            OwnedMessage::Pong(_) => {
//                info!("Received pong message from DISCORD. Skipping.");
//                None
//            }
//        }
//    }
//
//    fn on_text_message(&mut self, message: String) -> Option<OwnedMessage> {
//        debug!("Received message {:?}", &message);
//
//        let json: serde_json::Result<MessagePacket> = serde_json::from_str(&message);
//        debug!("Json is \n{:?}", &json);
//
//        match json {
//            Err(e) => {
//                error!("Failed to parse header json {}. Ignoring packet.", e);
//            },
//            Ok(content) => {
//                // Update last sequence
//                let p = &POOL;
//                self.last_sequence = content.s;
//
//                let opcode = OpCode::from(content.op);
//                match opcode {
//                    OpCode::Hello => {
//                        // Received Hello -> Register heartbeat and send my secrets
//
//                        let hello_response = IdentityPacket {
//                            token: p.key.clone(),
//                            properties: IdentityPropertiesPacket {
//                                os: Some("windows".to_string()),
//                                // TODO wtf am i doing?
//                                browser: Some("konno".to_string()),
//                                device: Some("konno".to_string()),
//                            },
//                            compress: None,
//                            large_threshold: None,
//                            shard: None,
//                            presence: None,
//                        };
//
//                        // Ignoring future
//                        //&self.sink.send(OwnedMessage::Text(serde_json::to_string(&hello_response).unwrap()));
//                    },
//                    _ => {
//                        unimplemented!("I dont know yet how to respond to {:?}", opcode)
//                    }
//                }
//            }
//        }
//        None
//    }
//
////    fn send(&self, message: &OwnedMessage) {
////        self.sink.clone_any_send_sync(message);
////    }
//}