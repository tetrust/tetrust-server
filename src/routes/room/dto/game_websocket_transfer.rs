use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::{RenderTheBoard, TakeMyBoard};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameWebsocketTransfer {
    TakeMyBoard(TakeMyBoard),       // input
    RenderTheBoard(RenderTheBoard), // output
}
