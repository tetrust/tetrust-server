use std::collections::HashMap;

use tokio::sync::broadcast;

use crate::routes::room::dto::GameWebsocketTransfer;

#[derive(Debug, Clone)]
pub struct GameState {
    pub tx_map: HashMap<String, broadcast::Sender<GameWebsocketTransfer>>,
}
