use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TakeMyBoard {
    pub room_id: ObjectId,
    pub board: Vec<i32>,
}
