use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::convert::From;

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

#[derive(Debug, Serialize, Deserialize)]
/// Message object
struct MessagePacket {
    /// Id
    pub id: u64,
    /// Id of the channel the message was sent in
    pub channel_id: u64,
    /// Id of the guild the message was sent in
    pub guild_id: u64,
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
    pub nonce: Option<u64>,
    /// Whether this message is pinned
    pub pinned: bool,
    /// If the message is generated by a webhook this is the webhook's id
    pub webhook_id: Option<u64>,
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
struct GuildMemberPacket {
    /// The user this guild member represents
    pub user: UserPacket,
    /// THe users guild nickname
    pub nick: Option<String>,
    /// Array of role object ids
    pub roles: Vec<u64>,
    /// Whe user joined the guild
    pub joined_at: String, // TODO this not a string
    /// Whether the user is deafened in voice channels
    pub deaf: bool,
    /// Whether the user is muted in voice channels
    pub mute: bool,
}

/// Role packet
#[derive(Debug, Serialize, Deserialize)]
struct RolePacket {
    /// Id
    pub id: u64,
    /// Role name
    pub name: String,
    /// Integer representation if hexadecimal color code
    pub color: i64,
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
    pub id: u64,
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
    pub id: u64,
    /// Emoji name
    pub name: String,
    /// Roles this emoji is ehitelisted to
    pub roles: Option<Vec<u64>>,
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
    pub id: u64,
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
enum MessageType {
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

// Has to be TryFrom, but it is unstable???
impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0 => MessageType::Default,
            1 => MessageType::RecipientAdd,
            2 => MessageType::RecipientRemove,
            3 => MessageType::Call,
            4 => MessageType::ChannelNameChange,
            5 => MessageType::ChannelIconMessage,
            6 => MessageType::ChannelPinnedMessage,
            7 => MessageType::GuildMemberJoin,
            _ => panic!("Unknown number for MessageType {}", value),
        }
    }
}

impl Serialize for MessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.into())
    }
}

// serde_json only works with u64, i64 and f64
// so lets try to deserialize u64 and convert to u8
impl<'de> Deserialize<'de> for MessageType {
    fn deserialize<D>(deserializer: D) -> Result<MessageType, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyLittleVisitor;

        impl<'de> Visitor<'de> for MyLittleVisitor {
            type Value = MessageType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("A number from 0 up to 7")
            }

            fn visit_u64<E>(self, s: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(MessageType::from(s as u8))
            }
        }
        deserializer.deserialize_u64(MyLittleVisitor)
    }
}

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
impl From<u8> for MessageActivityType {
    fn from(value: u8) -> Self {
        match value {
            1 => MessageActivityType::Join,
            2 => MessageActivityType::Spectate,
            3 => MessageActivityType::Listen,
            5 => MessageActivityType::JoinRequest,
            _ => panic!("Unknown number for MessageActivityType {}", value),
        }
    }
}

impl Serialize for MessageActivityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.into())
    }
}

// serde_json only works with u64, i64 and f64
// so lets try to deserialize u64 and convert to u8
impl<'de> Deserialize<'de> for MessageActivityType {
    fn deserialize<D>(deserializer: D) -> Result<MessageActivityType, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyLittleVisitor;

        impl<'de> Visitor<'de> for MyLittleVisitor {
            type Value = MessageActivityType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("A number 1, 2, 3 or 5")
            }

            fn visit_u64<E>(self, s: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(MessageActivityType::from(s as u8))
            }
        }
        deserializer.deserialize_u64(MyLittleVisitor)
    }
}
