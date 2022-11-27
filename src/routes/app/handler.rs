use std::sync::Arc;
use std::{collections::HashMap, error::Error};

use axum::{
    http::StatusCode,
    middleware,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Json, Router,
};
use mongodb::{bson::doc, options::CreateIndexOptions, IndexModel};
use mongodb::{options::IndexOptions, Database};

use crate::{
    extensions::{CurrentUser, MongoClient},
    models::{InsertRoomNumber, Room, RoomMember, RoomNumber, User},
};
use crate::{
    middlewares::auth_middleware,
    routes::{auth, room, user},
};

pub(crate) async fn router() -> Router {
    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .route("/init", get(init)) // 배포시 비활성화
        .nest("/user", user::router().await)
        .nest("/auth", auth::router().await)
        .nest("/room", room::router().await)
        .route_layer(middleware::from_fn(auth_middleware))
        .layer(Extension(MongoClient::get_database("tetrust").await));

    app
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

use super::dto::health_response::HealthReponse;

async fn health(
    database: Extension<Arc<Database>>,
    current_user: Extension<CurrentUser>,
) -> impl IntoResponse {
    let server_ok = true;
    let mut database_ok = false;
    let authorized = current_user.authorized;

    if let Ok(_collections) = database.list_collections(None, None).await {
        database_ok = true;
    }

    Json(HealthReponse {
        server_ok,
        database_ok,
        authorized,
    })
    .into_response()
}

// DB 기본 세팅
async fn init(database: Extension<Arc<Database>>) -> impl IntoResponse {
    let result: Result<(), Box<dyn Error>> = (|| async {
        let user = database.collection::<User>(User::NAME);
        user.create_index(IndexModel::builder().keys(doc! {"email": 1}).build(), None)
            .await?;

        let room = database.collection::<Room>(Room::NAME);
        room.create_index(
            IndexModel::builder().keys(doc! {"room_number": 1}).build(),
            None,
        )
        .await?;

        let room_number = database.collection::<RoomNumber>(RoomNumber::NAME);
        room_number
            .create_index(
                IndexModel::builder()
                    .keys(doc! {"room_number": 1})
                    .options(IndexOptions::builder().unique(true).build())
                    .build(),
                None,
            )
            .await?;

        if room_number.count_documents(None, None).await? == 0 {
            let room_number = database.collection::<InsertRoomNumber>(RoomNumber::NAME);

            for i in 0..=9999 {
                let number = format!("{:0>4}", i);
                let insert_data = InsertRoomNumber {
                    room_number: number,
                    in_used: false,
                };

                room_number.insert_one(insert_data, None).await?;
            }
        }

        let room_member = database.collection::<RoomMember>(RoomMember::NAME);
        room_member
            .create_index(
                IndexModel::builder()
                    .keys(doc! {"room_id": 1, "user_id":1})
                    .options(IndexOptions::builder().unique(true).build())
                    .build(),
                None,
            )
            .await?;

        Ok(())
    })()
    .await;

    match result {
        Ok(()) => StatusCode::OK.into_response(),
        Err(error) => {
            println!("error: {:?}", error);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
