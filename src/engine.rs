use log::{debug, error, info, warn};

use crate::data::POOL;
use crate::discord::*;
use actix::*;
use actix_web::ws::{Client, ClientWriter, Message, ProtocolError};
use crate::connector::{ClientMessage, WssConnector};
use futures::Future;


pub fn on_message(content: MessagePacket) {
    let opcode = OpCode::from(content.op);
    match opcode {
        OpCode::Hello => {
            hello(content);
        },
        OpCode::HeartbeatACK => {
            debug!("Heartbeat succeeded (recieved ACK)");
        },
        _ => {
            warn!("I dont know yet how to respond to {:?}", opcode)
        }
    }
}

/// Received Hello -> Register heartbeat and send my secrets
fn hello(content: MessagePacket) {
    let p = &POOL;
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
    let wss_con = System::current().registry().get::<WssConnector>();
    // TODO send?
    match wss_con.try_send(msg) {
        Ok(_) => {
            debug!("Succeeded delivering message")
        },
        Err(e) => {
            error!("Failed to deliver message: {}", e)
        }
    };
    // Register scheduler for heartbeat
    let p: HelloPacket = serde_json::from_value(content.d.unwrap()).unwrap();

    debug!("Configured heartbeat packet");

    let wss_con = System::current().registry().get::<WssConnector>();

    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(p.heartbeat_interval));
            debug!("It is time to send heartbeat packet");
            // No sync on heartbeat
            // TODO solve it
            let packet = MessagePacket {
                op: OpCode::Heartbeat.into(),
                d: None,
                s: None,
                t: None
            };

            let msg = ClientMessage {
                data: packet
            };

            match wss_con.try_send(msg) {
                Ok(_) => {
                    debug!("Succeeded delivering heartbeat message")
                },
                Err(e) => {
                    error!("Failed to deliver heartbeat message: {}", e)
                }
            }
        }
    });
}

/// Send heartbeat packet, acknowledging DISCORD that we are still alive.
fn heartbeat() {
    let packet = MessagePacket {
        op: OpCode::Heartbeat.into(),
        d: None,
        s: None,
        t: None
    };

    let msg = ClientMessage {
        data: packet
    };

    debug!("Configured heartbeat packet");

    let wss_con = System::current().registry().get::<WssConnector>();
    // TODO send?
    match wss_con.try_send(msg) {
        Ok(_) => {
            debug!("Succeeded delivering heartbeat message")
        },
        Err(e) => {
            error!("Failed to deliver heartbeat message: {}", e)
        }
    }
}