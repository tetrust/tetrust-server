use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// 게임방에 있는 멤버들

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomMember {
    pub _id: ObjectId,
    pub room_id: ObjectId, // 룸 번호
    pub active: bool,      // 게임 참여 활성화 상태
    pub on_play: bool,     // 게임 플레이 중
}

impl Room {
    pub const NAME: &'static str = "room_member";
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertRoomMember {
    pub room_id: ObjectId,
    pub active: bool,
    pub on_play: bool,
}
