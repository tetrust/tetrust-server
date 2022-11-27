use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TakeMyBoard {
    pub board: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderTheBoard {
    pub user_id: ObjectId,
    pub board: Vec<i32>,
}
