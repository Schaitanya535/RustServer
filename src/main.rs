// use axum::{routing::get, Router};
// use std::net::SocketAddr;

#[allow(dead_code)]
mod leet_code;
#[allow(dead_code)]
mod playground;
#[allow(dead_code)]
mod utils;

#[tokio::main]
async fn main() {
    leet_code::main()
    // playground::trying_flatten()

    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    // let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}
