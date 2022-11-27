use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameWebsocketOpenQuery {
    pub room_id: String,
}
