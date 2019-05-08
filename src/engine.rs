use log::{debug, error, info, warn};

use crate::connector::*;
use crate::data::POOL;
use crate::discord::*;
use actix::*;
use actix_web::client;
use actix_web::multipart::*;
use actix_web::HttpMessage;
use futures::stream::Stream;
use futures::Future;
use std::io::Write;
use std::thread::JoinHandle;

#[derive(Debug)]
pub struct Engine {
    /// Keep session id to be able to resume connection.
    session_id: Option<String>,
    /// Keep join point of heartbeat thread
    heartbeat_thread: Option<JoinHandle<()>>,
    /// Myself identifier
    myself_id: Option<Snowflake>,
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
                debug!("Heartbeat succeeded (received ACK)");
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

        let res = res.unwrap_or_else(|e| {
            panic!(
                "Failed to serialize IdentityPacket to respond to hello packet: {}",
                e
            )
        });

        let hello_response = WrapperPacket {
            op: OpCode::Identify,
            d: Some(res),
            s: None,
            t: None,
        };
        debug!("Created identity message: {:?}", &hello_response);
        let msg = ClientMessage {
            data: hello_response,
        };
        let wss_con = System::current().registry().get::<WssConnector>();
        Arbiter::spawn({
            wss_con
                .send(msg)
                .map_err(|e| {
                    error!("Something bad happened on sending request: {}", e);
                    ()
                })
                .map(|m| {
                    debug!("Succeeded delivering to WssConnector");
                    ()
                })
        });
        debug!("After Arbiter");
        // TODO send?
        //        match wss_con.try_send(msg) {
        //            Ok(_) => debug!("Succeeded delivering message"),
        //            Err(e) => error!("Failed to deliver message: {}", e),
        //        };
        // Register scheduler for heartbeat
        let hello_packet: HelloPacket = serde_json::from_value(content.d.unwrap()).unwrap();

        debug!("Configured heartbeat packet");

        let wss_con = System::current().registry().get::<WssConnector>();

        // TODO This has to be done with AsyncContext run_interval?
        let handle = std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_millis(
                    hello_packet.heartbeat_interval,
                ));
                debug!("It is time to send heartbeat packet");
                // No sync on heartbeat
                // Whatever, it works stable enough
                let packet = WrapperPacket {
                    op: OpCode::Heartbeat,
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

    /// Literally all regular events that happened on server side.
    fn dispatch(&mut self, content: WrapperPacket) {
        match &content.t {
            None => debug!("There was no \"t\" parameter in Dispatch event. Ignoring packet"),
            Some(t) => match t {
                Event::MessageCreate => {
                    debug!("Something was written in chat!");

                    match &self.myself_id {
                        None => warn!("I dont know who am i, so message was ignored"),
                        Some(_) => {
                            let message_packet: MessagePacket =
                                serde_json::from_value(content.d.unwrap()).unwrap();
                            debug!("The message is {:?}", &message_packet);
                            self.on_text_message(&message_packet);
                            self.inspect_file(&message_packet)
                        }
                    }
                }
                Event::Ready => {
                    debug!("Found Ready packet");
                    let ready_packet: ReadyPacket =
                        serde_json::from_value(content.d.unwrap()).unwrap();
                    self.myself_id = Some(ready_packet.user.id);
                    debug!("Myself id is {:?}", &self.myself_id);
                }
                _ => info!("We do not care about {:?} event. Ignoring packet", t),
            },
        }
    }

    fn on_text_message(&mut self, message_packet: &MessagePacket) {
        let author = &message_packet.author.id;
        let myself_id = self.myself_id.as_ref().unwrap();

        if author.eq(myself_id) {
            debug!("My own message. Ignoring packet");
            return;
        }

        let was_mentioned = message_packet.mentions.iter().any(|i| i.id.eq(myself_id));
        if !was_mentioned {
            debug!("I was not mentioned. Ignoring packet");
            return;
        }

        let my_mention = format!("<@{}>", myself_id.0);
        debug!("My mention is: {}", &my_mention);
        let mention_index = &message_packet.content.find(&my_mention);
        if mention_index.is_none() {
            warn!("Found myself in <mentions>, but could not detect myself in message text. Ignoring packet.");
            return;
        }

        let mention_index = mention_index.unwrap();
        let content = &message_packet.content[mention_index + my_mention.len()..];
        debug!("Content is: {}", content);

        // Respond with same text
        let channel_id = &message_packet.channel_id.0;
        let req_con = System::current().registry().get::<RequestConnector>();
        let author_id = &author.0;

        let request_data = serde_json::to_value(MessageRequestPacket::simple_text(&format!(
            "<@{}> {}",
            author_id, content
        )))
        .unwrap();
        let msg = RequestMessage {
            method: HttpMethod::POST,
            url: format!("/channels/{}/messages", channel_id),
            data: Some(request_data),
        };

        let res = req_con.try_send(msg);
        debug!("Message was sent to DISCORD? {:?}", res);
    }

    fn inspect_file(&mut self, message_packet: &MessagePacket) {
        if !message_packet.attachments.is_empty() {
            debug!("Found some attachments");
            for att in &message_packet.attachments {
                let name = att.filename.clone();
                let url = &att.url;
                debug!("File is {} from {}", name, url);

                let f = client::get(url).finish().unwrap().send();
                let f = f
                    .map_err(actix_web::error::Error::from)
                    .and_then(|resp| {
                        resp.body()
                            .limit(100 * 1024 * 1024)
                            .map_err(actix_web::error::Error::from)
                    })
                    .map(move |body| {
                        let curr_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
                        let mut file = std::fs::File::create(&format!("files/{}_{}", &curr_time, &name)).unwrap();
                        let res = file.write_all(body.as_ref());
                        debug!("Result of writing to file is {:?}", res);
                        ()
                    })
                    .map_err(|e| {
                        error!("Error happened {}", e);
                        ()
                    });
                Arbiter::spawn(f);
                //                tokio::spawn(f);
            }
        } else {
            debug!("No attachments found");
        }
    }
}
