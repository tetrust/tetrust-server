use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Query, State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post, put},
    Extension, Json, Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use mongodb::{bson::oid::ObjectId, Database};
use tokio::sync::broadcast;

use crate::{
    extensions::{mongo::MongoClient, CurrentUser, GameState},
    models::{room_number, InsertRoom, InsertRoomMember, InsertUser, RoomMember, User},
    routes::room::dto::{GameWebsocketTransfer, ReceiveGarbage, RenderTheBoard, TakeMyBoard},
    utils::{generate_uuid, hash_password},
};

use super::{
    dto::{
        CreateRoomRequest, CreateRoomResponse, EnterRoomRequest, EnterRoomResponse,
        GameWebsocketOpenQuery, StartRoomRequest, StartRoomResponse,
    },
    RoomService,
};

#[derive(Debug, Clone)]
pub struct GameStateFoo {}

pub async fn router() -> Router {
    let game_state = Arc::new(Mutex::new(GameState {
        tx_map: Default::default(),
    }));

    let app = Router::new()
        .route("/", post(create_room))
        .route("/enter", post(enter_room))
        .route("/start", put(start_room))
        .route("/websocket/game", get(handle_game))
        .with_state(game_state);

    app
}

async fn create_room(
    database: Extension<Arc<Database>>,
    current_user: Extension<CurrentUser>,
    Json(body): Json<CreateRoomRequest>,
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
    database: Extension<Arc<Database>>,
    current_user: Extension<CurrentUser>,
    Json(body): Json<EnterRoomRequest>,
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
    database: Extension<Arc<Database>>,
    current_user: Extension<CurrentUser>,
    Json(body): Json<StartRoomRequest>,
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

async fn handle_game(
    ws: WebSocketUpgrade,
    State(state): State<Arc<Mutex<GameState>>>,
    current_user: Extension<CurrentUser>,
    Query(query): Query<GameWebsocketOpenQuery>,
) -> impl IntoResponse {
    if current_user.authorized == false || current_user.user.is_none() {
        return (StatusCode::UNAUTHORIZED).into_response();
    }

    let user_id = current_user.user.as_ref().unwrap()._id;
    let room_id = query.room_id;

    if !state.lock().unwrap().tx_map.contains_key(&room_id) {
        let (tx, _rx) = broadcast::channel::<GameWebsocketTransfer>(100);
        state.lock().unwrap().tx_map.insert(room_id.clone(), tx);
    }

    let tx = state
        .lock()
        .unwrap()
        .tx_map
        .get(&room_id)
        .unwrap()
        .to_owned();

    ws.on_upgrade(move |socket: WebSocket| async move {
        let (mut sender, mut receiver) = socket.split();

        let mut rx = tx.subscribe();

        // Clone things we want to pass to the receiving task.
        let tx = tx.clone();

        // 웹소켓으로 입력이 들어오면 채널에 쏴주는 태스크
        let mut recv_task = tokio::spawn(async move {
            while let Some(Ok(incoming)) = receiver.next().await {
                match incoming {
                    Message::Text(text) => {
                        if let Ok(game_request) =
                            serde_json::from_str::<GameWebsocketTransfer>(text.as_str())
                        {
                            match game_request {
                                GameWebsocketTransfer::TakeMyBoard(data) => {
                                    println!(">> In TakeMyBoard");
                                    let data = RenderTheBoard {
                                        board: data.board,
                                        user_id,
                                    };

                                    tx.send(GameWebsocketTransfer::RenderTheBoard(data))
                                        .unwrap();
                                }
                                GameWebsocketTransfer::TakeGarbage(data) => {
                                    println!(">> In TakeGarbage");
                                    let data = ReceiveGarbage {
                                        line: data.line,
                                        user_id,
                                    };

                                    tx.send(GameWebsocketTransfer::ReceiveGarbage(data))
                                        .unwrap();
                                }
                                _ => {}
                            }
                        } else {
                            break;
                        }
                    }
                    _ => {
                        continue;
                    }
                }
            }
        });

        // 채널에 입력이 들어오면 웹소켓으로 쏴주는 태스크
        let mut send_task = tokio::spawn(async move {
            while let Ok(message) = rx.recv().await {
                match &message {
                    GameWebsocketTransfer::RenderTheBoard(data) => {
                        if data.user_id != user_id {
                            let text = serde_json::to_string(&message).unwrap();

                            if sender.send(Message::Text(text)).await.is_err() {
                                break;
                            }
                        }
                    }
                    GameWebsocketTransfer::ReceiveGarbage(data) => {
                        if data.user_id == user_id {
                            let text = serde_json::to_string(&message).unwrap();

                            if sender.send(Message::Text(text)).await.is_err() {
                                break;
                            }
                        }
                    }
                    _ => {}
                }
            }
        });

        // 태스크가 하나라도 끝나면 다른 것도 정리
        tokio::select! {
            _ = (&mut send_task) => recv_task.abort(),
            _ = (&mut recv_task) => send_task.abort(),
        };
    })
}
