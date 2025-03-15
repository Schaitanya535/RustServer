// use axum::{routing::get, Router};
// use std::net::SocketAddr;

#[allow(dead_code)]
mod leet_code;
#[allow(dead_code)]
mod playground;
#[allow(dead_code)]
mod utils;

mod stdio_transport;

mod lsp;

#[tokio::main]
async fn main() {
    leet_code::main();

    let rx = stdio_transport::StdioTransport::new().read_messages();

    for message in rx {
        // thread::spawn(move || {
        println!("Received message: {}", message);
        // })
    }

    // playground::trying_flatten()

    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    // let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}
