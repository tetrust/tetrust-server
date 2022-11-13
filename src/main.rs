mod routes;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let port = 3000;

    let app = routes::app::router();

    let address = SocketAddr::from(([0, 0, 0, 0], port));

    println!("listening on {}", address);
    axum::Server::bind(&address) // 주소 등록
        .serve(app.into_make_service()) // 라우터 등록 및 실행
        .await
        .unwrap();
}
