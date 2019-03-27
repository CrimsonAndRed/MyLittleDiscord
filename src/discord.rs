use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::convert::From;

/// General response from DISCORD.
#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePacket {
    // OpCode as u8
    pub op: u8,
    // Any internal data
    pub d: serde_json::Value,
    // Session number
    pub s: Option<i64>,
    // Event name
    pub t: Option<String>,
}

/// First response from Discord WebSocket.
#[derive(Debug, Serialize, Deserialize)]
pub struct HelloPacket {
    // Heartbeat interval in milliseconds
    pub heartbeat_interval: u64,
    // Some meta information from DISCORD.
    pub _trace: Vec<String>,
}

/// Heartbeat packet to maintain connection to DISCORD.
#[derive(Debug, Serialize, Deserialize)]
pub struct HeartbeatPacket {
    // OpCode - shounld be 1
    pub op: u8,
    // last s received by me
    pub d: Option<i64>,
}

/// Packet to indetify myself to DISCORD.
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityPacket {
    // My secret
    pub token: String,
    // Some properties
    pub properties: IdentityPropertiesPacket,
    // Whether this connection supports compression of packets
    pub compress: Option<bool>,
    // Offline members of guild threshold
    pub large_threshold: Option<u64>,
    // Something to deal with extra large bots
    pub shard: Option<Vec<u64>>,
    // My status
    pub presence: Option<UpdateStatusPacket>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityPropertiesPacket {
    #[serde(alias = "$os")]
    pub os: Option<String>,

    #[serde(alias = "$browser")]
    pub browser: Option<String>,

    #[serde(alias = "$device")]
    pub device: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStatusPacket {
    pub since: Option<u64>,
    pub game: Option<HashMap<String, String>>, // TODO struct
    pub status: Status,
    pub afk: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Online,
    Dnd,
    Idle,
    Invisible,
    Offline,
}

/// Opcodes of DISOCRD protocol.
#[derive(Debug)]
pub enum OpCode {
    Dispatch,
    Heartbeat,
    Identify,
    StatusUpdate,
    VoiceStateUpdate,
    Resume,
    Reconnect,
    RequestGuildMembers,
    InvalidSession,
    Hello,
    HeartbeatACK,
}

// Some converters for OpCode
impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        match self {
            OpCode::Dispatch => 0,
            OpCode::Heartbeat => 1,
            OpCode::Identify => 2,
            OpCode::StatusUpdate => 3,
            OpCode::VoiceStateUpdate => 4,
            OpCode::Resume => 6,
            OpCode::Reconnect => 7,
            OpCode::RequestGuildMembers => 8,
            OpCode::InvalidSession => 9,
            OpCode::Hello => 10,
            OpCode::HeartbeatACK => 11,
        }
    }
}

// Has to bee TryFrom, but it is unstable???
impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::Dispatch,
            1 => OpCode::Heartbeat,
            2 => OpCode::Identify,
            3 => OpCode::StatusUpdate,
            4 => OpCode::VoiceStateUpdate,
            6 => OpCode::Resume,
            7 => OpCode::Reconnect,
            8 => OpCode::RequestGuildMembers,
            9 => OpCode::InvalidSession,
            10 => OpCode::Hello,
            11 => OpCode::HeartbeatACK,
            _ => panic!("Unknown number for OpCode {}", value),
        }
    }
}