use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// 기본 게임방 모델

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub _id: ObjectId,
    pub title: String,
    pub room_number: String,
}

impl Room {
    pub const NAME: &'static str = "room";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertRoom {}
