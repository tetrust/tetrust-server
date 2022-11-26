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
    current_user: Extension<Arc<CurrentUser>>,
) -> impl IntoResponse {
    if current_user.authorized == false || current_user.user.is_none() {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    let current_user = current_user.user.as_ref().unwrap().to_owned();

    let service = RoomService::new(database);
    let mut response = CreateRoomResponse { room_number: None };

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
        players: vec![],
        waiting_list: vec![],
    };

    match service.create_room(room_data).await {
        Ok(user_id) => {
            response.room_number = Some(room_number);
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(error) => {
            println!("error: {:?}", error);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
