use serde::{Deserialize, Serialize}

/// First response from Discord WebSocket.
#[derive(Debug, Serialize, Deserialize)]
struct HelloPacket {
    heartbeat_interval: u64,
    _trace: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct IdentityPacket {
    token: String,
    properties: Map<String, String>,
    compress: Option<bool>,
    large_threshold: Option<u64>,
    shard: Option<Vec<u64>>,
    presence: 
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateStatusPacket {
    since: Option<u64>,
    game: Option<Map<String, String>>, // TODO struct
    status: Status,
    afk: bool,
}

#[derive(Serialize, Deserialize)]
enum Status {
    Online = "online",
    Dnd = "dnd",
    Idle = "idle",
    invisible = "invisible",
    Offline = "offline",
}