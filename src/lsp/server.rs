use std::{fs::OpenOptions, io::Write};

use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer};

pub struct Backend {
    pub client: Client,
    pub log_file_path: String,
}

impl Backend {
    fn log(&self, message: &str) {
        let utc_time = chrono::Utc::now();

        let log_entry = format!(
            "{timestamp}, {bytes} bytes: {message}\n",
            timestamp = utc_time,
            bytes = message.len(),
            message = message
        );

        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file_path)
        {
            let _ = file.write_all(log_entry.as_bytes());
        } else {
            eprintln!("Failed to write to log file");
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        self.client
            .log_message(MessageType::INFO, "Server initialized!")
            .await;

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "Example LSP".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.log("Initialized");
        self.client
            .log_message(MessageType::INFO, "Server fully initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        self.log("Shutting down");
        self.client
            .log_message(MessageType::INFO, "Server shutting down.")
            .await;
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;

        self.client
            .log_message(
                MessageType::INFO,
                format!("File opened: {}\nContents: {}", uri, text),
            )
            .await;
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let position = params.text_document_position_params.position;
        self.log(format!("Hover requested at position: {:?}", position).as_str());
        self.client
            .log_message(
                MessageType::INFO,
                format!("Hover requested at position: {:?}", position),
            )
            .await;

        let contents = HoverContents::Scalar(MarkedString::String("Hover!".to_string()));

        Ok(Some(Hover {
            contents,
            range: None,
        }))
    }
}
