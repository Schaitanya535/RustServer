// use axum::{routing::get, Router};
// use std::net::SocketAddr;

use lsp::server::Backend;
use tower_lsp::{LspService, Server};

#[allow(dead_code)]
mod leet_code;
#[allow(dead_code)]
mod playground;
#[allow(dead_code)]
mod utils;

#[allow(dead_code)]
mod stdio_transport;

#[allow(unused_variables)]
mod lsp;

#[allow(dead_code)]
mod data_structures;

#[tokio::main]
async fn main() {
    // leet_code::main();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        log_file_path: "/Users/chaitanyasura/projects/RustServer/log.txt".to_string(),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
    // let rx = stdio_transport::StdioTransport::new().read_messages();

    // for message in rx {
    //     // thread::spawn(move || {
    //     println!("Received message: {}", message);
    //     // })
    // }
}
