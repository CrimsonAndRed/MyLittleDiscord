use log::{debug, error, info, warn};

use crate::connector::{ClientMessage, WssConnector};
use crate::data::POOL;
use crate::discord::*;
use actix::*;
use std::thread::JoinHandle;

#[derive(Debug)]
pub struct Engine {
    /// Keep session id to be able to resume connection.
    session_id: Option<String>,
    /// Keep join point of heartbeat thread
    heartbeat_thread: Option<JoinHandle<()>>,
    /// Myself identifier
    myself_id: Option<u64>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            session_id: None,
            heartbeat_thread: None,
            myself_id: None,
        }
    }
    // Lets assume that:
    // DISCORD: <The internet is a scary place. Disconnections happen, especially with persistent connections.> - we ignore this statement, our internet is peaceful-friendly and stable.
    pub fn on_message(&mut self, content: WrapperPacket) {
        match &content.op {
            OpCode::Hello => {
                self.hello(content);
            }
            OpCode::Dispatch => {
                self.dispatch(content);
            }
            OpCode::HeartbeatACK => {
                debug!("Heartbeat succeeded (recieved ACK)");
            }
            _ => warn!("I dont know yet how to respond to {:?}", &content.op),
        }
    }

    /// Received Hello -> Register heartbeat and send my secrets
    fn hello(&mut self, content: WrapperPacket) {
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
        let hello_response = WrapperPacket {
            op: OpCode::Identify.into(),
            // TODO no unwrap
            d: Some(res.unwrap()),
            s: None,
            t: None,
        };
        debug!("Created identity message: {:?}", &hello_response);
        let msg = ClientMessage {
            data: hello_response,
        };
        let wss_con = System::current().registry().get::<WssConnector>();
        // TODO send?
        match wss_con.try_send(msg) {
            Ok(_) => debug!("Succeeded delivering message"),
            Err(e) => error!("Failed to deliver message: {}", e),
        };
        // Register scheduler for heartbeat
        let p: HelloPacket = serde_json::from_value(content.d.unwrap()).unwrap();

        debug!("Configured heartbeat packet");

        let wss_con = System::current().registry().get::<WssConnector>();

        let handle = std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_millis(p.heartbeat_interval));
                debug!("It is time to send heartbeat packet");
                // No sync on heartbeat
                // TODO solve it
                let packet = WrapperPacket {
                    op: OpCode::Heartbeat.into(),
                    d: None,
                    s: None,
                    t: None,
                };

                let msg = ClientMessage { data: packet };

                // TODO send?
                match wss_con.try_send(msg) {
                    Ok(_) => debug!("Succeeded delivering heartbeat message"),
                    Err(e) => error!("Failed to deliver heartbeat message: {}", e),
                }
            }
        });

        self.heartbeat_thread = Some(handle);
    }

    /// Send heartbeat packet, acknowledging DISCORD that we are still alive.
    fn heartbeat(&self) {
        let packet = WrapperPacket {
            op: OpCode::Heartbeat,
            d: None,
            s: None,
            t: None,
        };

        let msg = ClientMessage { data: packet };

        debug!("Configured heartbeat packet");

        let wss_con = System::current().registry().get::<WssConnector>();
        // TODO send?
        match wss_con.try_send(msg) {
            Ok(_) => debug!("Succeeded delivering heartbeat message"),
            Err(e) => error!("Failed to deliver heartbeat message: {}", e),
        }
    }

    /// Literally all regular events happened on server side.
    fn dispatch(&mut self, content: WrapperPacket) {
        match &content.t {
            None => debug!("There was no \"t\" parameter in Dispatch event. Ignoring packet"),
            Some(t) => match t {
                Event::MessageCreate => {
                    debug!("Something was written in chat!");
                }
                _ => info!("We do not care about {:?} event. Ignoring packet", t),
            },
        }
    }
}
