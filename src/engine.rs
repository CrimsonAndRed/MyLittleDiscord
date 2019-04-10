use log::{debug, error, info, warn};

use crate::data::POOL;
use crate::discord::IdentityPacket;
use crate::discord::IdentityPropertiesPacket;
use crate::discord::MessagePacket;
use crate::discord::OpCode;
use actix::*;
use actix_web::ws::{Client, ClientWriter, Message, ProtocolError};


pub fn on_message(content: MessagePacket) {
    let p = &POOL;
    let opcode = OpCode::from(content.op);
    match opcode {
        OpCode::Hello => {
            // Received Hello -> Register heartbeat and send my secrets

            let identity_packet = IdentityPacket {
                token: p.key.clone(),
                properties: IdentityPropertiesPacket {
                    os: Some("windows".to_string()),
                    // TODO wtf am i doing?
                    browser: Some("konno".to_string()),
                    device: Some("konno".to_string()),
                },
                compress: None,
                large_threshold: None,
                shard: None,
                presence: None,
            };
            let res = serde_json::to_value(&identity_packet);
            let hello_response = MessagePacket {
                op: OpCode::Identify.into(),
                // TODO no unwrap
                d: Some(res.unwrap()),
                s: None,
                t: None
            };
            debug!("Sending to discord: {:?}", hello_response);

            // Ignoring future
            //&self.sink.send(OwnedMessage::Text(serde_json::to_string(&hello_response).unwrap()));
        },
        _ => {
            unimplemented!("I dont know yet how to respond to {:?}", opcode)
        }
    }
}

pub fn heartbeat() {
    let packet = MessagePacket {
        op: OpCode::Heartbeat.into(),
        d: None,
        s: None,
        t: None
    };
    // TODO
}

//    fn send(&self, message: &OwnedMessage) {
//        self.sink.clone_any_send_sync(message);
//    }

