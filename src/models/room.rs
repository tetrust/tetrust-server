use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// 기본 게임방 모델

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub _id: ObjectId,
    pub title: String,       // 방 제목
    pub room_number: String, // 접속용 고유 방 번호: 0000-9999
    pub is_private: bool,    // 비공개방인지 여부
    pub host_id: ObjectId,   // 방 주인
    pub on_play: bool,       // 플레이중인지
}

impl Room {
    pub const NAME: &'static str = "room";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertRoom {
    pub title: String,
    pub room_number: String,
    pub is_private: bool,
    pub host_id: ObjectId,
    pub on_play: bool,
}
