use std::sync::Arc;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::State,
    response::Response,
    routing::get,
    Extension, Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use mongodb::Database;
use tokio::sync::broadcast;

use crate::extensions::{CurrentUser, GameState};

pub(crate) fn router() -> Router {
    let (tx, _rx) = broadcast::channel(100);

    let game_state = Arc::new(GameState { tx });

    // 라우터 생성
    let app = Router::new()
        .route("/game", get(handle_game))
        .with_state(game_state);

    app
}

async fn handle_game(
    ws: WebSocketUpgrade,
    State(state): State<Arc<GameState>>,
    current_user: Extension<CurrentUser>,
) -> Response {
    ws.on_upgrade(move |mut socket: WebSocket| async move {
        let (mut sender, mut receiver) = socket.split();

        while let Some(message) = receiver.next().await {
            let message = if let Ok(message) = message {
                message
            } else {
                // client disconnected
                return;
            };

            if let Ok(text) = message.into_text() {
                println!("{:?}", text);

                let message = Message::Text("foo".into());

                if sender.send(message).await.is_err() {
                    // client disconnected
                    return;
                }
            }
        }
    })
}
