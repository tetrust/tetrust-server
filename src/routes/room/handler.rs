use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router,
};
use mongodb::Database;

use crate::{
    extensions::{mongo::MongoClient, CurrentUser},
    models::{room_number, InsertRoom, InsertUser, User},
    utils::{generate_uuid, hash_password},
};

use super::{
    dto::{CreateRoomRequest, CreateRoomResponse},
    RoomService,
};

pub async fn router() -> Router {
    let app = Router::new().route("/", post(create_room));

    app
}

async fn create_room(
    Json(body): Json<CreateRoomRequest>,
    database: Extension<Arc<Database>>,
    current_user: Extension<CurrentUser>,
) -> impl IntoResponse + Send {
    if current_user.authorized == false || current_user.user.is_none() {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    let current_user = current_user.user.as_ref().unwrap().to_owned();

    let service = RoomService::new(database.clone());
    let mut response = CreateRoomResponse {
        room_number: None,
        room_id: None,
    };

    let room_number = service.take_room_number().await;

    let room_number = match room_number {
        Ok(room_number) => match room_number {
            Some(room_number) => room_number,
            None => {
                println!("no more room number");
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        },
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let room_data = InsertRoom {
        title: body.title,
        is_private: body.is_private,
        room_number: room_number.clone(),
        players: vec![current_user._id],
        waiting_list: vec![],
        host_id: current_user._id,
    };

    let service = RoomService::new(database);

    match service.create_room(room_data).await {
        Ok(room_id) => {
            response.room_number = Some(room_number);
            response.room_id = Some(room_id);
            Json(response).into_response()
        }
        Err(error) => {
            println!("error: {:?}", error);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
