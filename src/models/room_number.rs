use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// 기본 게임방 할당번호 관리 모델

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomNumber {
    pub _id: ObjectId,
    pub room_number: String, // 방 번호: 0000-9999
    pub in_used: bool,       // 사용중인지 여부
}

impl RoomNumber {
    pub const NAME: &'static str = "room_number";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertRoomNumber {
    pub room_number: String,
    pub in_used: bool,
}
