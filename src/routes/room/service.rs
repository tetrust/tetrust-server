use std::{str::FromStr, sync::Arc};

use axum::Extension;
use mongodb::{
    bson::{doc, oid::ObjectId, Array},
    Database,
};
use std::error::Error;

use crate::models::{InsertRoom, InsertRoomNumber, InsertUser, Room, RoomNumber, User};

pub struct RoomService {
    database: Extension<Arc<Database>>,
}

impl RoomService {
    pub fn new(database: Extension<Arc<Database>>) -> Self {
        Self { database }
    }

    pub async fn take_room_number(&self) -> Result<Option<String>, Box<dyn Error>> {
        let room_number = self.database.collection::<RoomNumber>(RoomNumber::NAME);

        // 추후에는 랜덤액세스로 가져오는 편이 좋지 않을까 생각중
        let result = room_number.find_one(doc! {"in_used": false}, None).await?;

        if let Some(ref update_data) = result {
            room_number
                .update_one(doc! {"_id": update_data._id}, doc! {"in_used":true}, None)
                .await?;
        }

        Ok(result.map(|e| e.room_number))
    }

    pub async fn put_back_room_number(&self, number: String) -> Result<(), Box<dyn Error>> {
        let room_number = self.database.collection::<RoomNumber>(RoomNumber::NAME);

        let result = room_number
            .find_one(doc! {"room_number": number}, None)
            .await?;

        if let Some(ref update_data) = result {
            room_number
                .update_one(doc! {"_id": update_data._id}, doc! {"in_used":false}, None)
                .await?;
        }

        Ok(())
    }

    pub async fn create_room(&self, room_data: InsertRoom) -> Result<String, Box<dyn Error>> {
        let user = self.database.collection::<InsertRoom>(Room::NAME);

        let result = user.insert_one(room_data, None).await?;
        let room_id = result.inserted_id.as_object_id().unwrap();
        let room_id = room_id.to_hex();

        Ok(room_id)
    }
}
