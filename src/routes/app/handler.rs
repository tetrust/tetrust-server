use std::sync::Arc;

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Extension, Json, Router,
};
use mongodb::Database;

use crate::extensions::mongo::MongoClient;
use crate::routes::{user, websocket};

pub(crate) async fn router() -> Router {
    // 라우터 생성
    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .nest("/user", user::router().await)
        .nest("/websocket", websocket::router())
        .layer(Extension(MongoClient::get_database("tetrust").await));

    app
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

use super::dto::health_response::HealthReponse;

async fn health(database: Extension<Arc<Database>>) -> impl IntoResponse {
    let server_ok = true;
    let mut database_ok = false;

    if let Ok(_collections) = database.list_collections(None, None).await {
        database_ok = true;
    }

    Json(HealthReponse {
        server_ok,
        database_ok,
    })
    .into_response()
}
