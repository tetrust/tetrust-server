use std::sync::Arc;

use axum::{
    middleware,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Json, Router,
};
use mongodb::Database;

use crate::extensions::{CurrentUser, MongoClient};
use crate::{
    middlewares::auth_middleware,
    routes::{auth, user, websocket},
};

pub(crate) async fn router() -> Router {
    // 라우터 생성
    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health))
        .nest("/user", user::router().await)
        .nest("/auth", auth::router().await)
        .nest("/websocket", websocket::router())
        .route_layer(middleware::from_fn(auth_middleware))
        .layer(Extension(MongoClient::get_database("tetrust").await))
        .layer(Extension(Arc::new(CurrentUser::default())));

    app
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

use super::dto::health_response::HealthReponse;

async fn health(
    database: Extension<Arc<Database>>,
    current_user: Extension<Arc<CurrentUser>>,
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
