mod extension;
mod routes;

use std::net::SocketAddr;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().expect("read .env failed");

    let port = std::env::var("PORT")
        .ok()
        .map(|e| e.parse::<u16>().ok())
        .flatten()
        .unwrap_or(3000);

    let app = routes::app::router();

    let address = SocketAddr::from(([0, 0, 0, 0], port));

    println!("listening on {}", address);
    axum::Server::bind(&address) // 주소 등록
        .serve(app.await.into_make_service()) // 라우터 등록 및 실행
        .await
        .unwrap();
}
