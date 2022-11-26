use std::{str::FromStr, sync::Arc};

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post, put},
    Extension, Json, Router,
};
use mongodb::{bson::oid::ObjectId, Database};

use crate::{
    extensions::{mongo::MongoClient, CurrentUser},
    models::{room_number, InsertRoom, InsertRoomMember, InsertUser, RoomMember, User},
    utils::{generate_uuid, hash_password},
};

use super::{
    dto::{
        CreateRoomRequest, CreateRoomResponse, EnterRoomRequest, EnterRoomResponse,
        StartRoomRequest, StartRoomResponse,
    },
    RoomService,
};

pub async fn router() -> Router {
    let app = Router::new()
        .route("/", post(create_room))
        .route("/enter", post(enter_room))
        .route("/start", put(start_room));

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
        host_id: current_user._id,
        on_play: false,
    };

    let service = RoomService::new(database);

    match service.create_room(room_data).await {
        Ok(room_id) => {
            response.room_number = Some(room_number);
            response.room_id = Some(room_id.clone());

            let member_data = InsertRoomMember {
                room_id: ObjectId::from_str(room_id.as_str()).unwrap(),
                user_id: current_user._id,
                active: true,
                on_play: false,
            };

            if let Err(error) = service.create_room_member(member_data).await {
                println!("error: {:?}", error);
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }

            Json(response).into_response()
        }
        Err(error) => {
            println!("error: {:?}", error);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

async fn enter_room(
    Json(body): Json<EnterRoomRequest>,
    database: Extension<Arc<Database>>,
    current_user: Extension<CurrentUser>,
) -> impl IntoResponse + Send {
    if current_user.authorized == false || current_user.user.is_none() {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    let current_user = current_user.user.as_ref().unwrap().to_owned();

    let service = RoomService::new(database.clone());
    let response = EnterRoomResponse {};

    let room = match service.find_by_room_number(body.room_number).await {
        Ok(room) => match room {
            Some(room) => room,
            None => {
                return (StatusCode::NOT_FOUND).into_response();
            }
        },
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    let member_data = InsertRoomMember {
        room_id: room._id,
        user_id: current_user._id,
        active: true,
        on_play: false,
    };

    if let Err(error) = service.create_room_member(member_data).await {
        println!("error: {:?}", error);
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    Json(response).into_response()
}

async fn start_room(
    Json(body): Json<StartRoomRequest>,
    database: Extension<Arc<Database>>,
    current_user: Extension<CurrentUser>,
) -> impl IntoResponse + Send {
    if current_user.authorized == false || current_user.user.is_none() {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    let current_user = current_user.user.as_ref().unwrap().to_owned();

    let service = RoomService::new(database.clone());
    let response = StartRoomResponse {};

    let room = match service.find_by_room_number(body.room_number).await {
        Ok(room) => match room {
            Some(room) => room,
            None => {
                return (StatusCode::NOT_FOUND).into_response();
            }
        },
        Err(error) => {
            println!("error: {:?}", error);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    // HOST만 시작 가능
    if room.host_id != current_user._id {
        return (StatusCode::FORBIDDEN).into_response();
    }

    if let Err(error) = service.start_room(room._id).await {
        println!("error: {:?}", error);
        return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
    }

    Json(response).into_response()
}
