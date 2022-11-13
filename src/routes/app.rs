use axum::{response::Html, routing::get, Router};

pub(crate) fn router() -> Router {
    // 라우터 생성
    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health));

    app
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn health() -> Html<&'static str> {
    Html("OK")
}
