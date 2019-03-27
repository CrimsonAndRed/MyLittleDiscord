use log::{debug, info};
use websocket::OwnedMessage;

use crate::discord::MessagePacket;
use crate::discord::OpCode;

pub fn on_discord_message(message: OwnedMessage) -> Option<OwnedMessage> {
    match message {
        OwnedMessage::Close(e) => {
            info!("Received close message from DISCORD");
            Some(OwnedMessage::Close(e))
        }
        OwnedMessage::Ping(d) => {
            info!("Received pong message from DISCORD");
            Some(OwnedMessage::Pong(d))
        }
        OwnedMessage::Text(text) => {
            info!("Received text message from DISCORD");
            on_text_message(text);
            None
        }
        OwnedMessage::Binary(_) => {
            info!("Received binary message from DISCORD. Skipping.");
            None
        }
        OwnedMessage::Pong(_) => {
            info!("Received pong message from DISCORD. Skipping.");
            None
        }
    }
}

fn on_text_message(message: String) {
    debug!("Received message {:?}", &message);

    let json: serde_json::Result<MessagePacket> = serde_json::from_str(&message);
    debug!("Json is \n{:?}", &json);

    if let Ok(content) = json {
        let num = content.op;

        debug!("Original opcode is {}", num);
        let opcode = OpCode::from(num);
        debug!("Parsed opcode is {:?}", opcode);
        let num2: u8 = opcode.into();
        debug!("Returned to opcode {}", num2);
    }
}
