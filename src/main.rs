// use axum::{routing::get, Router};
// use std::net::SocketAddr;

mod leet_code;

#[tokio::main]
async fn main() {
    use leet_code::leet_code;
    leet_code();

    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    // let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}
