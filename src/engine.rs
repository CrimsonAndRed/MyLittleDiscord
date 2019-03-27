use websocket::OwnedMessage;
use log::{debug};
use websocket::ws::dataframe::DataFrame;

use super::discord::OpCode;
//use std::convert::From;

pub fn on_discord_message(message: OwnedMessage) -> Option<OwnedMessage> {
    
    debug!("Received message {:?}", &message);

    let num = message.opcode();
    debug!("Original num us {}", num);
    let opcode = OpCode::from(num);

    debug!("Got opcode {:?}", opcode);
    let num2: u8 = opcode.into();
    debug!("Got num {}", num2);
    


    match message {
        OwnedMessage::Close(e) => Some(OwnedMessage::Close(e)),
        OwnedMessage::Ping(d) => Some(OwnedMessage::Pong(d)),
        _ => None,
    }
}