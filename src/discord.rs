use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::convert::{Into, TryFrom};

/// This macro implements Serialize and Deserialize for c-like enums (java-like for me), that implements Into and TryFrom u8.
macro_rules! simple_serde_enum_to_u8 {
    ($impl_type:ty, $expected_text:expr) => {
        impl Serialize for $impl_type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_u8(self.into())
            }
        }

        // serde_json only works with u64, i64 and f64
        // so lets try to deserialize u64 and convert to u8
        impl<'de> Deserialize<'de> for $impl_type {
            fn deserialize<D>(deserializer: D) -> Result<$impl_type, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct MyLittleVisitor;

                impl<'de> Visitor<'de> for MyLittleVisitor {
                    type Value = $impl_type;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str($expected_text)
                    }

                    fn visit_u64<E>(self, s: u64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        <$impl_type>::try_from(s as u8).map_err(serde::de::Error::custom)
                    }
                }
                deserializer.deserialize_u64(MyLittleVisitor)
            }
        }
    };
}

/// This macro implements Serialize and Deserialize for enums, that implements Into and TryFrom str.
macro_rules! simple_serde_enum_to_str {
    ($impl_type:ty, $expected_text:expr) => {
        impl Serialize for $impl_type {
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
        impl<'de> Deserialize<'de> for $impl_type {
            fn deserialize<D>(deserializer: D) -> Result<$impl_type, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct MyLittleVisitor;

                impl<'de> Visitor<'de> for MyLittleVisitor {
                    type Value = $impl_type;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str($expected_text)
                    }

                    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        <$impl_type>::try_from(s).map_err(serde::de::Error::custom)
                    }
                }
                deserializer.deserialize_str(MyLittleVisitor)
            }
        }
    };
}

/// Snowflake is String in JSON representation, but it always has to be 64bit integer.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Snowflake(pub String);

impl Into<u64> for &Snowflake {
    fn into(self) -> u64 {
        self.0.parse().unwrap()
    }
}

/// General response from DISCORD.
#[derive(Debug, Serialize, Deserialize)]
pub struct WrapperPacket {
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
    /// Defines the heartbeat interval
    Hello,
    /// Contains the initial state information
    Ready,
    /// Response to Resume
    Resumed,
    /// Failure response to Identify or Resume or invalid active session
    InvalidSession,
    /// New channel created
    ChannelCreate,
    /// Channel was updated
    ChannelUpdate,
    /// Channel was deleted
    ChannelDelete,
    /// Message was pinned or unpinned
    ChannelPinsUpdate,
    /// Lazy-load for unavailable guild, guild became available, or user joined a new guild
    GuildCreate,
    /// Guild was updated
    GuildUpdate,
    /// Guild became unavailable, or user left/was removed from a guild
    GuildDelete,
    /// User was banned from a guild
    GuildBanAdd,
    /// User was unbanned from a guild
    GuildBanRemove,
    /// Guild emojis were updated
    GuildEmojisUpdate,
    /// Guild integration was updated
    GuildIntegrationsUpdate,
    /// New user joined a guild
    GuildMemberAdd,
    /// User was removed from a guild
    GuildMemberRemove,
    /// Guild member was updated
    GuildMemberUpdate,
    /// Response to Request Guild Members
    GuildMembersChunk,
    /// Guild role was created
    GuildRoleCreate,
    /// Guild role was updated
    GuildRoleUpdate,
    /// Guild role was deleted
    GuildRoleDelete,
    /// Message was created
    MessageCreate,
    /// Message was edited
    MessageUpdate,
    /// Message was deleted
    MessageDelete,
    /// Multiple messages were deleted at once
    MessageDeleteBulk,
    /// User reacted to a message
    MessageReactionAdd,
    /// User removed a reaction from a message
    MessageReactionRemove,
    /// All reactions were explicitly removed from a message
    MessageReactionRemoveAll,
    /// User was updated
    PresenceUpdate,
    /// User started typing in a channel
    TypingStart,
    /// Properties about the user changed
    UserUpdate,
    /// Someone joined, left, or moved a voice channel
    VoiceStateUpdate,
    /// Guild's voice server was updated
    VoiceServerUpdate,
    /// Guild channel webhook was created, update, or deleted
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
    /// User's new status
    pub status: Status,
    /// Whether or not the client is afk
    pub afk: bool,
}

/// User status object
#[derive(Debug)]
pub enum Status {
    Online,
    Dnd,
    Idle,
    Invisible,
    Offline,
}

impl Into<String> for Status {
    fn into(self) -> String {
        match self {
            Status::Online => "online".to_owned(),
            Status::Dnd => "dnd".to_owned(),
            Status::Idle => "idle".to_owned(),
            Status::Invisible => "invisible".to_owned(),
            Status::Offline => "offline".to_owned(),
        }
    }
}

impl Into<String> for &Status {
    fn into(self) -> String {
        match self {
            Status::Online => "online".to_owned(),
            Status::Dnd => "dnd".to_owned(),
            Status::Idle => "idle".to_owned(),
            Status::Invisible => "invisible".to_owned(),
            Status::Offline => "offline".to_owned(),
        }
    }
}

// Has to be TryFrom, but it is unstable???
impl TryFrom<&str> for Status {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "online" => Ok(Status::Online),
            "dnd" => Ok(Status::Dnd),
            "idle" => Ok(Status::Idle),
            "invisible" => Ok(Status::Invisible),
            "offline" => Ok(Status::Offline),
            _ => Err(format!("Unknown number for MessageActivityType {}", value)),
        }
    }
}

simple_serde_enum_to_str!(Status, "Status name from DISCORD");

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
impl TryFrom<u8> for OpCode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::Dispatch),
            1 => Ok(OpCode::Heartbeat),
            2 => Ok(OpCode::Identify),
            3 => Ok(OpCode::StatusUpdate),
            4 => Ok(OpCode::VoiceStateUpdate),
            6 => Ok(OpCode::Resume),
            7 => Ok(OpCode::Reconnect),
            8 => Ok(OpCode::RequestGuildMembers),
            9 => Ok(OpCode::InvalidSession),
            10 => Ok(OpCode::Hello),
            11 => Ok(OpCode::HeartbeatACK),
            _ => Err(format!("Unknown number for OpCode {}", value)),
        }
    }
}

simple_serde_enum_to_u8!(OpCode, "A number from 0 up to 11");

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

impl TryFrom<&str> for Event {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "HELLO" => Ok(Event::Hello),
            "READY" => Ok(Event::Ready),
            "RESUMED" => Ok(Event::Resumed),
            "INVALID_SESSION" => Ok(Event::InvalidSession),
            "CHANNEL_CREATE" => Ok(Event::ChannelCreate),
            "CHANNEL_UPDATE" => Ok(Event::ChannelUpdate),
            "CHANNEL_DELETE" => Ok(Event::ChannelDelete),
            "CHANNEL_PINS_UPDATE" => Ok(Event::ChannelPinsUpdate),
            "GUILD_CREATE" => Ok(Event::GuildCreate),
            "GUILD_UPDATE" => Ok(Event::GuildUpdate),
            "GUILD_DELETE" => Ok(Event::GuildDelete),
            "GUILD_BAN_ADD" => Ok(Event::GuildBanAdd),
            "GUILD_BAN_REMOVE" => Ok(Event::GuildBanRemove),
            "GUILD_EMOJIS_UPDATE" => Ok(Event::GuildEmojisUpdate),
            "GUILD_INTEGRATIONS_UPDATE" => Ok(Event::GuildIntegrationsUpdate),
            "GUILD_MEMBER_ADD" => Ok(Event::GuildMemberAdd),
            "GUILD_MEMBER_REMOVE" => Ok(Event::GuildMemberRemove),
            "GUILD_MEMBER_UPDATE" => Ok(Event::GuildMemberUpdate),
            "GUILD_MEMBERS_CHUNK" => Ok(Event::GuildMembersChunk),
            "GUILD_ROLE_CREATE" => Ok(Event::GuildRoleCreate),
            "GUILD_ROLE_UPDATE" => Ok(Event::GuildRoleUpdate),
            "GUILD_ROLE_DELETE" => Ok(Event::GuildRoleDelete),
            "MESSAGE_CREATE" => Ok(Event::MessageCreate),
            "MESSAGE_UPDATE" => Ok(Event::MessageUpdate),
            "MESSAGE_DELETE" => Ok(Event::MessageDelete),
            "MESSAGE_DELETE_BULK" => Ok(Event::MessageDeleteBulk),
            "MESSAGE_REACTION_ADD" => Ok(Event::MessageReactionAdd),
            "MESSAGE_REACTION_REMOVE" => Ok(Event::MessageReactionRemove),
            "MESSAGE_REACTION_REMOVE_ALL" => Ok(Event::MessageReactionRemoveAll),
            "PRESENCE_UPDATE" => Ok(Event::PresenceUpdate),
            "TYPING_START" => Ok(Event::TypingStart),
            "USER_UPDATE" => Ok(Event::UserUpdate),
            "VOICE_STATE_UPDATE" => Ok(Event::VoiceStateUpdate),
            "VOICE_SERVER_UPDATE" => Ok(Event::VoiceServerUpdate),
            "WEBHOOKS_UPDATE" => Ok(Event::WebhooksUpdate),
            _ => Err(format!("Unknown event name {}", value)),
        }
    }
}
simple_serde_enum_to_str!(Event, "Event name from DISCORD");

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
    pub shard: Option<Vec<u64>>,
}

/// Unavailable guilds.
#[derive(Debug, Serialize, Deserialize)]
pub struct UnavailableGuildPacket {
    /// Guild id
    pub id: Snowflake,
    /// Flag ?
    pub unavailable: bool,
}

/// User object
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPacket {
    /// Id
    pub id: Snowflake,
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

#[derive(Debug, Serialize, Deserialize)]
/// Message object
pub struct MessagePacket {
    /// Id
    pub id: Snowflake,
    /// Id of the channel the message was sent in
    pub channel_id: Snowflake,
    /// Id of the guild the message was sent in
    pub guild_id: Snowflake,
    /// The author of this message
    pub author: UserPacket,
    /// Member properties for this message's author
    pub member: GuildMemberPacket,
    /// Contents of the messages
    pub content: String,
    /// When this message was sent
    pub timestamp: String, // TODO it is not string
    /// When this nessages was editted. or null if never
    pub edited_timestamp: Option<String>, // TODO it is not string
    /// Whether this was a TTS message
    pub tts: bool,
    /// Whether tgis message mentions everyone
    pub mention_everyone: bool,
    /// Users specigically nentioned in this message
    pub mentions: Vec<UserPacket>, // TODO add member field in UserPacket
    /// Roles specigically mentioned in this message
    pub mention_roles: Vec<RolePacket>,
    /// Any attached files
    pub attachments: Vec<AttachmentPacket>,
    /// Any embedded content
    pub embeds: Vec<EmbedPacket>,
    /// Reactions to the message
    pub reactions: Option<Vec<ReactionPacket>>,
    /// Used for validation a message was sent
    pub nonce: Option<Snowflake>,
    /// Whether this message is pinned
    pub pinned: bool,
    /// If the message is generated by a webhook this is the webhook's id
    pub webhook_id: Option<Snowflake>,
    /// Type if message
    #[serde(alias = "type")]
    pub message_type: MessageType,
    /// Sent with Rich Presence related chat embeds
    pub activity: Option<MessageActivityPacket>,
    /// Sent with Rich Presence related chat embeds
    pub application: Option<MessageApplicationPacket>,
}

/// Information about guild members
#[derive(Debug, Serialize, Deserialize)]
pub struct GuildMemberPacket {
    /// The user this guild member represents
    pub user: Option<UserPacket>,
    /// THe users guild nickname
    pub nick: Option<String>,
    /// Array of role object ids
    pub roles: Vec<Snowflake>,
    /// Whe user joined the guild
    pub joined_at: String, // TODO this not a string
    /// Whether the user is deafened in voice channels
    pub deaf: bool,
    /// Whether the user is muted in voice channels
    pub mute: bool,
}

/// Role packet
#[derive(Debug, Serialize, Deserialize)]
pub struct RolePacket {
    /// Id
    pub id: Snowflake,
    /// Role name
    pub name: String,
    /// Integer representation if hexadecimal color code
    pub color: i64, // TODO not an i64, but color
    /// If this role is pinned in user listing
    pub hoist: bool,
    /// Position of this role
    pub position: i64,
    /// Permission bit set
    pub permissions: i64,
    /// Whether this role is managed by an integration
    pub managed: bool,
    /// Whether this role is mentionable
    pub mentionable: bool,
}

/// Attachment to message
#[derive(Debug, Serialize, Deserialize)]
pub struct AttachmentPacket {
    /// Id
    pub id: Snowflake,
    /// Name of attached file
    pub filename: String,
    /// Size of file in bytes
    pub size: i64,
    /// Source url of file
    pub url: String,
    /// A proxied url of file
    pub proxy_url: String,
    /// Height of file (if image)
    pub height: Option<i64>,
    /// Width of file (if image)
    pub width: Option<i64>,
}

/// Embed object
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedPacket {
    /// Title of embed
    pub title: Option<String>,
    /// Type of embed (always "rich" for webhook embeds)
    #[serde(alias = "type")]
    pub embed_type: Option<String>,
    /// Description of embed
    pub description: Option<String>,
    ///Url of embed
    pub url: Option<String>,
    /// Timestamp of embed content
    pub timestamp: Option<String>, //TODO not a string
    /// Color code of the embed
    pub color: Option<i64>,
    /// Footer information
    pub footer: Option<EmbedFooterPacket>,
    /// Image information
    pub image: Option<EmbedImagePacket>,
    /// Thumbnail information
    pub thumbnail: Option<EmbedThumbnailPacket>,
    /// Video information
    pub video: Option<EmbedVideoPacket>,
    /// Provider information
    pub provider: Option<EmbedProviderPacket>,
    /// Author information
    pub author: Option<EmbedAuthorPacket>,
    /// Fields information
    pub fields: Option<Vec<EmbedFieldPacket>>,
}

/// Embed footer
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedFooterPacket {
    /// Footer text
    pub text: String,
    /// Url of footer icon (only supports http(s) and attachments)
    pub icon_url: Option<String>,
    /// A proxied url of footer icon
    pub proxy_icon_url: Option<String>,
}

/// Embed image
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedImagePacket {
    /// Source url of image (only supports http(s) and attachments)
    pub url: Option<String>,
    /// A proxied url of the image
    pub proxy_url: Option<String>,
    /// Height of image
    pub height: Option<i64>,
    /// Width of image
    pub width: Option<i64>,
}

/// Embed thumbnail
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedThumbnailPacket {
    /// Source url of thumbnail (only supports http(s) and attachments)
    pub url: Option<String>,
    /// A proxied url of the thumbnail
    pub proxy_url: Option<String>,
    /// Height of thumbnail
    pub height: Option<i64>,
    /// Width pf thumbnail
    pub width: Option<i64>,
}

/// Embed video
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedVideoPacket {
    /// Source url of video
    pub url: Option<String>,
    /// Height of video
    pub height: Option<i64>,
    /// Width of video
    pub width: Option<i64>,
}

/// Embed provider
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedProviderPacket {
    /// Name of provider
    pub name: Option<String>,
    /// Url of provider
    pub url: Option<String>,
}

/// Embed author
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedAuthorPacket {
    /// Name of author
    pub name: Option<String>,
    /// Url of author
    pub url: Option<String>,
    /// Url of author icon (only supports http(s) and attachments)
    pub icon_url: Option<String>,
    /// A proxied url of author icon
    pub proxy_icon_url: Option<String>,
}

/// Embed field
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedFieldPacket {
    /// Name of field
    pub name: String,
    /// Value of field
    pub value: String,
    /// Whether or not this field display inline
    pub inline: Option<bool>,
}

/// Reaction object
#[derive(Debug, Serialize, Deserialize)]
pub struct ReactionPacket {
    /// Times this emoji has been used to react
    pub count: i64,
    /// Whether hte current user reacted using this emoji
    pub me: bool,
    /// Emoji information
    pub emoji: EmojiPacket,
}

/// Emoji object
#[derive(Debug, Serialize, Deserialize)]
pub struct EmojiPacket {
    /// Emoji id
    pub id: Snowflake,
    /// Emoji name
    pub name: String,
    /// Roles this emoji is ehitelisted to
    pub roles: Option<Vec<Snowflake>>,
    /// User that created this emoji
    pub user: Option<UserPacket>,
    /// Whether this emoji must nbe wrapped in colons
    pub require_colons: Option<bool>,
    /// Whether this emoji is managed
    pub managed: Option<bool>,
    /// Whether this emoji is animated
    pub animated: Option<bool>,
}

/// Message Activity object
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageActivityPacket {
    #[serde(alias = "type")]
    /// Type of message activity
    pub activity_type: MessageActivityType,
    /// Party_id from rich presence event
    pub party_id: Option<String>,
}

/// Message application object
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageApplicationPacket {
    /// Id of application
    pub id: Snowflake,
    /// Id of the embed's image asset
    pub cover_image: Option<String>,
    /// Application description
    pub description: String,
    /// Id of the application's icon
    pub icon: String,
    /// Name of the application
    pub name: String,
}

#[derive(Debug)]
pub enum MessageType {
    Default,
    RecipientAdd,
    RecipientRemove,
    Call,
    ChannelNameChange,
    ChannelIconMessage,
    ChannelPinnedMessage,
    GuildMemberJoin,
}

// Some converters for MessageType
impl Into<u8> for MessageType {
    fn into(self) -> u8 {
        match self {
            MessageType::Default => 0,
            MessageType::RecipientAdd => 1,
            MessageType::RecipientRemove => 2,
            MessageType::Call => 3,
            MessageType::ChannelNameChange => 4,
            MessageType::ChannelIconMessage => 5,
            MessageType::ChannelPinnedMessage => 6,
            MessageType::GuildMemberJoin => 7,
        }
    }
}

impl Into<u8> for &MessageType {
    fn into(self) -> u8 {
        match self {
            MessageType::Default => 0,
            MessageType::RecipientAdd => 1,
            MessageType::RecipientRemove => 2,
            MessageType::Call => 3,
            MessageType::ChannelNameChange => 4,
            MessageType::ChannelIconMessage => 5,
            MessageType::ChannelPinnedMessage => 6,
            MessageType::GuildMemberJoin => 7,
        }
    }
}

impl TryFrom<u8> for MessageType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MessageType::Default),
            1 => Ok(MessageType::RecipientAdd),
            2 => Ok(MessageType::RecipientRemove),
            3 => Ok(MessageType::Call),
            4 => Ok(MessageType::ChannelNameChange),
            5 => Ok(MessageType::ChannelIconMessage),
            6 => Ok(MessageType::ChannelPinnedMessage),
            7 => Ok(MessageType::GuildMemberJoin),
            _ => Err(format!("Unknown number for MessageType {}", value)),
        }
    }
}

simple_serde_enum_to_u8!(MessageType, "A number from 0 up to 7");

#[derive(Debug)]
pub enum MessageActivityType {
    Join,
    Spectate,
    Listen,
    JoinRequest,
}

// Some converters for MessageActivityType
impl Into<u8> for MessageActivityType {
    fn into(self) -> u8 {
        match self {
            MessageActivityType::Join => 1,
            MessageActivityType::Spectate => 2,
            MessageActivityType::Listen => 3,
            MessageActivityType::JoinRequest => 5,
        }
    }
}

impl Into<u8> for &MessageActivityType {
    fn into(self) -> u8 {
        match self {
            MessageActivityType::Join => 1,
            MessageActivityType::Spectate => 2,
            MessageActivityType::Listen => 3,
            MessageActivityType::JoinRequest => 5,
        }
    }
}

// Has to be TryFrom, but it is unstable???
impl TryFrom<u8> for MessageActivityType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MessageActivityType::Join),
            2 => Ok(MessageActivityType::Spectate),
            3 => Ok(MessageActivityType::Listen),
            5 => Ok(MessageActivityType::JoinRequest),
            _ => Err(format!("Unknown number for MessageActivityType {}", value)),
        }
    }
}

simple_serde_enum_to_u8!(MessageActivityType, "A number 1, 2, 3 or 5");
// TODO have to write structures for different events

// Requests
/// Create my own messages
#[derive(Debug, Serialize)]
pub struct MessageRequestPacket {
    /// Text content of message
    pub content: Option<String>,
    /// Nonce that can be used for optinistic message sending
    pub nonce: Option<Snowflake>,
    /// true if it is TTS message
    pub tts: bool,
    /// content of file being sent
    pub file: Option<Vec<u8>>,
    /// embedded rich text
    pub embed: Option<EmbedPacket>,
    /// JSON encoded
    pub payload_json: Option<serde_json::Value>,
}

impl MessageRequestPacket {
    pub fn simple_text(text: &str) -> Self {
        MessageRequestPacket {
            content: Some(text.to_string()),
            nonce: None,
            tts: false,
            file: None,
            embed: None,
            payload_json: None,
        }
    }
}
