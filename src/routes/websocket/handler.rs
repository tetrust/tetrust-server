use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};

pub(crate) fn router() -> Router {
    // 라우터 생성
    let app = Router::new().route("/game", get(handle_game));

    app
}

async fn handle_game(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_game_socket)
}

async fn handle_game_socket(mut socket: WebSocket) {
    while let Some(message) = socket.recv().await {
        let message = if let Ok(message) = message {
            message
        } else {
            // client disconnected
            return;
        };

        if let Ok(text) = message.into_text() {
            println!("{:?}", text);

            let message = Message::Text("foo".into());

            if socket.send(message).await.is_err() {
                // client disconnected
                return;
            }
        }
    }
}
