use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TakeGarbage {
    pub line: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiveGarbage {
    pub user_id: ObjectId,
    pub line: u32,
}
