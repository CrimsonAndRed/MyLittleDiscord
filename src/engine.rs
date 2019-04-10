use log::{debug, error, info, warn};

use crate::data::POOL;
use crate::discord::IdentityPacket;
use crate::discord::IdentityPropertiesPacket;
use crate::discord::MessagePacket;
use crate::discord::OpCode;
use actix::*;
use actix_web::ws::{Client, ClientWriter, Message, ProtocolError};
use crate::connector::{ClientMessage, MyLittleConnection};
use futures::Future;


pub fn on_message(content: MessagePacket) {
    let p = &POOL;
    let opcode = OpCode::from(content.op);
    match opcode {
        OpCode::Hello => {
            // Received Hello -> Register heartbeat and send my secrets

            let identity_packet = IdentityPacket {
                token: p.key.clone(),
                properties: IdentityPropertiesPacket {
                    os: "windows".to_owned(),
                    // TODO wtf am i doing?
                    browser: "konno".to_owned(),
                    device: "konno".to_owned(),
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
            debug!("Created identity message: {:?}", &hello_response);
            let msg = ClientMessage {
                data: hello_response,
            };
            let mlc = System::current().registry().get::<MyLittleConnection>();
            // TODO send?
            mlc.try_send(msg);
        },
        _ => {
            warn!("I dont know yet how to respond to {:?}", opcode)
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