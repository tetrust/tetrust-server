use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::{ReceiveGarbage, RenderTheBoard, TakeGarbage, TakeMyBoard};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameWebsocketTransfer {
    TakeMyBoard(TakeMyBoard),       // input
    RenderTheBoard(RenderTheBoard), // output
    TakeGarbage(TakeGarbage),       // input
    ReceiveGarbage(ReceiveGarbage), // output
}
