use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// 기본 게임방 모델

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub _id: ObjectId,
    pub title: String,               // 방 제목
    pub room_number: String,         // 접속용 고유 방 번호: 0000-9999
    pub players: Vec<ObjectId>,      // 게임 플레이어 목록
    pub waiting_list: Vec<ObjectId>, // 대기자 목록
    pub is_private: bool,            // 비공개방인지 여부
}

impl Room {
    pub const NAME: &'static str = "room";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertRoom {
    pub title: String,
    pub room_number: String,
    pub players: Vec<ObjectId>,
    pub waiting_list: Vec<ObjectId>,
    pub is_private: bool,
}
