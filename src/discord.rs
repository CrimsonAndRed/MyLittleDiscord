use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::convert::From;

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePacket {
    pub op: u8,
    pub d: serde_json::Value,
    pub s: Option<i64>,
    pub t: Option<String>,
}

/// First response from Discord WebSocket.
#[derive(Debug, Serialize, Deserialize)]
pub struct HelloPacket {
    heartbeat_interval: u64,
    _trace: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityPacket {
    token: String,
    properties: HashMap<String, String>,
    compress: Option<bool>,
    large_threshold: Option<u64>,
    shard: Option<Vec<u64>>,
    presence: UpdateStatusPacket,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStatusPacket {
    since: Option<u64>,
    game: Option<HashMap<String, String>>, // TODO struct
    status: Status,
    afk: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Online,
    Dnd,
    Idle,
    Invisible,
    Offline,
}

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
