use log::{debug, info, error, warn};

use crate::discord::MessagePacket;
use crate::discord::OpCode;
use crate::data::POOL;
use crate::discord::IdentityPacket;
use crate::discord::IdentityPropertiesPacket;
use actix_web::ws::{Client, ClientWriter, Message, ProtocolError};
use actix::*;

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