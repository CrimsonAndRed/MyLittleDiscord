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
    pub t: Option<Event>,
}

/// Event for payload in Dispatch packets.
#[derive(Debug)]
pub enum Event {
    /// defines the heartbeat interval
    Hello,
    /// contains the initial state information
    Ready,
    /// response to Resume
    Resumed,
    /// failure response to Identify or Resume or invalid active session
    InvalidSession,
    /// new channel created
    ChannelCreate,
    /// channel was updated
    ChannelUpdate,
    /// channel was deleted
    ChannelDelete,
    /// message was pinned or unpinned
    ChannelPinsUpdate,
    /// lazy-load for unavailable guild, guild became available, or user joined a new guild
    GuildCreate,
    /// guild was updated
    GuildUpdate,
    /// guild became unavailable, or user left/was removed from a guild
    GuildDelete,
    /// user was banned from a guild
    GuildBanAdd,
    /// user was unbanned from a guild
    GuildBanRemove,
    /// guild emojis were updated
    GuildEmojisUpdate,
    /// guild integration was updated
    GuildIntegrationsUpdate,
    /// new user joined a guild
    GuildMemberAdd,
    /// user was removed from a guild
    GuildMemberRemove,
    /// guild member was updated
    GuildMemberUpdate,
    /// response to Request Guild Members
    GuildMembersChunk,
    /// guild role was created
    GuildRoleCreate,
    /// guild role was updated
    GuildRoleUpdate,
    /// guild role was deleted
    GuildRoleDelete,
    /// message was created
    MessageCreate,
    /// message was edited
    MessageUpdate,
    /// message was deleted
    MessageDelete,
    /// multiple messages were deleted at once
    MessageDeleteBulk,
    /// user reacted to a message
    MessageReactionAdd,
    /// user removed a reaction from a message
    MessageReactionRemove,
    /// all reactions were explicitly removed from a message
    MessageReactionRemoveAll,
    /// user was updated
    PresenceUpdate,
    /// user started typing in a channel
    TypingStart,
    /// properties about the user changed
    UserUpdate,
    /// someone joined, left, or moved a voice channel
    VoiceStateUpdate,
    /// guild's voice server was updated
    VoiceServerUpdate,
    /// guild channel webhook was created, update, or deleted
    WebhooksUpdate,
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
        serializer.serialize_u8(self.into())
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

impl Into<String> for Event {
    fn into(self) -> String {
        match self {
            Event::Hello => "HELLO".to_owned(),
            Event::Ready => "READY".to_owned(),
            Event::Resumed => "RESUMED".to_owned(),
            Event::InvalidSession => "INVALID_SESSION".to_owned(),
            Event::ChannelCreate => "CHANNEL_CREATE".to_owned(),
            Event::ChannelUpdate => "CHANNEL_UPDATE".to_owned(),
            Event::ChannelDelete => "CHANNEL_DELETE".to_owned(),
            Event::ChannelPinsUpdate => "CHANNEL_PINS_UPDATE".to_owned(),
            Event::GuildCreate => "GUILD_CREATE".to_owned(),
            Event::GuildUpdate => "GUILD_UPDATE".to_owned(),
            Event::GuildDelete => "GUILD_DELETE".to_owned(),
            Event::GuildBanAdd => "GUILD_BAN_ADD".to_owned(),
            Event::GuildBanRemove => "GUILD_BAN_REMOVE".to_owned(),
            Event::GuildEmojisUpdate => "GUILD_EMOJIS_UPDATE".to_owned(),
            Event::GuildIntegrationsUpdate => "GUILD_INTEGRATIONS_UPDATE".to_owned(),
            Event::GuildMemberAdd => "GUILD_MEMBER_ADD".to_owned(),
            Event::GuildMemberRemove => "GUILD_MEMBER_REMOVE".to_owned(),
            Event::GuildMemberUpdate => "GUILD_MEMBER_UPDATE".to_owned(),
            Event::GuildMembersChunk => "GUILD_MEMBERS_CHUNK".to_owned(),
            Event::GuildRoleCreate => "GUILD_ROLE_CREATE".to_owned(),
            Event::GuildRoleUpdate => "GUILD_ROLE_UPDATE".to_owned(),
            Event::GuildRoleDelete => "GUILD_ROLE_DELETE".to_owned(),
            Event::MessageCreate => "MESSAGE_CREATE".to_owned(),
            Event::MessageUpdate => "MESSAGE_UPDATE".to_owned(),
            Event::MessageDelete => "MESSAGE_DELETE".to_owned(),
            Event::MessageDeleteBulk => "MESSAGE_DELETE_BULK".to_owned(),
            Event::MessageReactionAdd => "MESSAGE_REACTION_ADD".to_owned(),
            Event::MessageReactionRemove => "MESSAGE_REACTION_REMOVE".to_owned(),
            Event::MessageReactionRemoveAll => "MESSAGE_REACTION_REMOVE_ALL".to_owned(),
            Event::PresenceUpdate => "PRESENCE_UPDATE".to_owned(),
            Event::TypingStart => "TYPING_START".to_owned(),
            Event::UserUpdate => "USER_UPDATE".to_owned(),
            Event::VoiceStateUpdate => "VOICE_STATE_UPDATE".to_owned(),
            Event::VoiceServerUpdate => "VOICE_SERVER_UPDATE".to_owned(),
            Event::WebhooksUpdate => "WEBHOOKS_UPDATE".to_owned(),
        }
    }
}

// Some converters for Event
impl Into<String> for &Event {
    fn into(self) -> String {
        match self {
            Event::Hello => "HELLO".to_owned(),
            Event::Ready => "READY".to_owned(),
            Event::Resumed => "RESUMED".to_owned(),
            Event::InvalidSession => "INVALID_SESSION".to_owned(),
            Event::ChannelCreate => "CHANNEL_CREATE".to_owned(),
            Event::ChannelUpdate => "CHANNEL_UPDATE".to_owned(),
            Event::ChannelDelete => "CHANNEL_DELETE".to_owned(),
            Event::ChannelPinsUpdate => "CHANNEL_PINS_UPDATE".to_owned(),
            Event::GuildCreate => "GUILD_CREATE".to_owned(),
            Event::GuildUpdate => "GUILD_UPDATE".to_owned(),
            Event::GuildDelete => "GUILD_DELETE".to_owned(),
            Event::GuildBanAdd => "GUILD_BAN_ADD".to_owned(),
            Event::GuildBanRemove => "GUILD_BAN_REMOVE".to_owned(),
            Event::GuildEmojisUpdate => "GUILD_EMOJIS_UPDATE".to_owned(),
            Event::GuildIntegrationsUpdate => "GUILD_INTEGRATIONS_UPDATE".to_owned(),
            Event::GuildMemberAdd => "GUILD_MEMBER_ADD".to_owned(),
            Event::GuildMemberRemove => "GUILD_MEMBER_REMOVE".to_owned(),
            Event::GuildMemberUpdate => "GUILD_MEMBER_UPDATE".to_owned(),
            Event::GuildMembersChunk => "GUILD_MEMBERS_CHUNK".to_owned(),
            Event::GuildRoleCreate => "GUILD_ROLE_CREATE".to_owned(),
            Event::GuildRoleUpdate => "GUILD_ROLE_UPDATE".to_owned(),
            Event::GuildRoleDelete => "GUILD_ROLE_DELETE".to_owned(),
            Event::MessageCreate => "MESSAGE_CREATE".to_owned(),
            Event::MessageUpdate => "MESSAGE_UPDATE".to_owned(),
            Event::MessageDelete => "MESSAGE_DELETE".to_owned(),
            Event::MessageDeleteBulk => "MESSAGE_DELETE_BULK".to_owned(),
            Event::MessageReactionAdd => "MESSAGE_REACTION_ADD".to_owned(),
            Event::MessageReactionRemove => "MESSAGE_REACTION_REMOVE".to_owned(),
            Event::MessageReactionRemoveAll => "MESSAGE_REACTION_REMOVE_ALL".to_owned(),
            Event::PresenceUpdate => "PRESENCE_UPDATE".to_owned(),
            Event::TypingStart => "TYPING_START".to_owned(),
            Event::UserUpdate => "USER_UPDATE".to_owned(),
            Event::VoiceStateUpdate => "VOICE_STATE_UPDATE".to_owned(),
            Event::VoiceServerUpdate => "VOICE_SERVER_UPDATE".to_owned(),
            Event::WebhooksUpdate => "WEBHOOKS_UPDATE".to_owned(),
        }
    }
}

// Has to be TryFrom, but it is unstable???
impl From<&str> for Event {
    fn from(value: &str) -> Self {
        match value {
            "HELLO" => Event::Hello,
            "READY" => Event::Ready,
            "RESUMED" => Event::Resumed,
            "INVALID_SESSION" => Event::InvalidSession,
            "CHANNEL_CREATE" => Event::ChannelCreate,
            "CHANNEL_UPDATE" => Event::ChannelUpdate,
            "CHANNEL_DELETE" => Event::ChannelDelete,
            "CHANNEL_PINS_UPDATE" => Event::ChannelPinsUpdate,
            "GUILD_CREATE" => Event::GuildCreate,
            "GUILD_UPDATE" => Event::GuildUpdate,
            "GUILD_DELETE" => Event::GuildDelete,
            "GUILD_BAN_ADD" => Event::GuildBanAdd,
            "GUILD_BAN_REMOVE" => Event::GuildBanRemove,
            "GUILD_EMOJIS_UPDATE" => Event::GuildEmojisUpdate,
            "GUILD_INTEGRATIONS_UPDATE" => Event::GuildIntegrationsUpdate,
            "GUILD_MEMBER_ADD" => Event::GuildMemberAdd,
            "GUILD_MEMBER_REMOVE" => Event::GuildMemberRemove,
            "GUILD_MEMBER_UPDATE" => Event::GuildMemberUpdate,
            "GUILD_MEMBERS_CHUNK" => Event::GuildMembersChunk,
            "GUILD_ROLE_CREATE" => Event::GuildRoleCreate,
            "GUILD_ROLE_UPDATE" => Event::GuildRoleUpdate,
            "GUILD_ROLE_DELETE" => Event::GuildRoleDelete,
            "MESSAGE_CREATE" => Event::MessageCreate,
            "MESSAGE_UPDATE" => Event::MessageUpdate,
            "MESSAGE_DELETE" => Event::MessageDelete,
            "MESSAGE_DELETE_BULK" => Event::MessageDeleteBulk,
            "MESSAGE_REACTION_ADD" => Event::MessageReactionAdd,
            "MESSAGE_REACTION_REMOVE" => Event::MessageReactionRemove,
            "MESSAGE_REACTION_REMOVE_ALL" => Event::MessageReactionRemoveAll,
            "PRESENCE_UPDATE" => Event::PresenceUpdate,
            "TYPING_START" => Event::TypingStart,
            "USER_UPDATE" => Event::UserUpdate,
            "VOICE_STATE_UPDATE" => Event::VoiceStateUpdate,
            "VOICE_SERVER_UPDATE" => Event::VoiceServerUpdate,
            "WEBHOOKS_UPDATE" => Event::WebhooksUpdate,
            _ => panic!("Unknown event name {}", value),
        }
    }
}

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let x: String = self.into();
        serializer.serialize_str(&x)
    }
}

// serde_json only works with u64, i64 and f64
// so lets try to deserialize u64 and convert to u8
impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Event, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyLittleVisitor;

        impl<'de> Visitor<'de> for MyLittleVisitor {
            type Value = Event;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("A string as <From Event>")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Event::from(s))
            }
        }
        deserializer.deserialize_str(MyLittleVisitor)
    }
}

/// Ready packet.
/// Received when identification is completed.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReadyPacket {
    /// Protocol version
    pub v: u8,
    /// User object
    pub user: UserPacket,
    /// Empty array (legacy support?)
    pub private_channels: Vec<String>,
    /// Unavailable guilds
    pub guilds: Vec<UnavailableGuildPacket>,
    /// Session id for resuming connection
    pub session_id: String,
    /// debugging information
    pub _trace: Vec<String>,
    /// Shard information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<Vec<i64>>,
}

/// Unavailable guilds.
#[derive(Debug, Serialize, Deserialize)]
pub struct UnavailableGuildPacket {
    /// Guild id
    pub id: u64,
    /// Flag ?
    pub unavailable: bool,
}

/// User object
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPacket {
    /// Id
    pub id: u64,
    /// Username
    pub username: String,
    ///Discriminator (4-digit tag)
    pub discriminator: String,
    /// Avatar hash
    pub avatar: Option<String>,
    /// Bot flag
    pub bot: Option<bool>,
    /// Two factor auth enabled flag
    pub mfa_enabled: Option<bool>,
    /// Locale
    pub locale: Option<String>,
    /// Verified flag
    pub verified: Option<bool>,
    /// Email
    pub email: Option<String>,
    /// User flags
    pub flags: Option<i64>,
    /// Premium
    pub premium_type: Option<i64>,
}
