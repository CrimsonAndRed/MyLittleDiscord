use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::convert::From;

/// General response from DISCORD.
#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePacket {
    /// OpCode as u8
    pub op: OpCode,
    /// Any internal data
    pub d: Option<serde_json::Value>,
    /// Session number
    pub s: Option<i64>,
    /// Event name
    pub t: Option<String>,
}

/// First response from Discord WebSocket.
#[derive(Debug, Serialize, Deserialize)]
pub struct HelloPacket {
    /// Heartbeat interval in milliseconds
    pub heartbeat_interval: u64,
    /// Some meta information from DISCORD.
    pub _trace: Vec<String>,
}

/// Packet to identify myself to DISCORD.
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityPacket {
    /// My secret
    pub token: String,
    /// Some properties
    pub properties: IdentityPropertiesPacket,
    /// Whether this connection supports compression of packets
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,
    /// Offline members of guild threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_threshold: Option<u64>,
    /// Something to deal with extra large bots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<Vec<u64>>,
    /// My status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<UpdateStatusPacket>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityPropertiesPacket {
    /// My operating system
    #[serde(alias = "$os")]
    pub os: String,
    /// My library name
    #[serde(alias = "$browser")]
    pub browser: String,
    /// My library name
    #[serde(alias = "$device")]
    pub device: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStatusPacket {
    /// Unix time in milliseconds of when the client went idle, or null if the client is not idle
    pub since: Option<u64>,
    /// null, or user's new activity
    pub game: Option<HashMap<String, String>>, // TODO struct
    /// user's new status
    pub status: Status,
    /// whether or not the client is afk
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

/// Opcodes of DISCORD protocol.
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

impl Into<u8> for &OpCode {
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

// Has to be TryFrom, but it is unstable???
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

impl Serialize for OpCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let num: u8 = self.into();
        serializer.serialize_u8(num)
    }
}

// serde_json only works with u64, i64 and f64
// so lets try to deserialize u64 and convert to u8
impl<'de> Deserialize<'de> for OpCode {
    fn deserialize<D>(deserializer: D) -> Result<OpCode, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyLittleVisitor;

        impl<'de> Visitor<'de> for MyLittleVisitor {
            type Value = OpCode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("A number from 0 up to 11")
            }

            fn visit_u64<E>(self, s: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(OpCode::from(s as u8))
            }
        }
        deserializer.deserialize_u64(MyLittleVisitor)
    }
}
