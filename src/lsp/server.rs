use crate::lsp::parser::parser;
use crate::lsp::src_tree::*;

use dashmap::mapref::one::Ref;
use dashmap::DashMap;
use std::sync::RwLock;
use std::{fs::OpenOptions, io::Write};

use chrono::format;
use serde::Serialize;
use tower_lsp::{
    jsonrpc::Result,
    lsp_types::{
        notification::DidChangeTextDocument,
        request::{GotoDeclarationParams, GotoDeclarationResponse},
        *,
    },
    Client, LanguageServer,
};

pub struct Backend {
    client: Client,
    workspace: RwLock<Option<Url>>,
    document_map: DashMap<Url, SrcTree>,
    log_file_path: String,
}

impl Backend {
    fn log(&self, message: &str) {
        let language = tree_sitter_sscript::LANGUAGE;
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

impl Backend {
    async fn on_change(&self, params: TextDocumentItem) {
        // TODO: Support incremental update
        let src_tree = SrcTree::new(params.text);
        let diagnostics = src_tree.diagnostics();

        self.document_map.insert(params.uri.clone(), src_tree);
        self.client
            .publish_diagnostics(params.uri, diagnostics, Some(params.version))
            .await;
    }

    fn url(&self, path: &str) -> Option<Url> {
        self.workspace
            .read()
            .unwrap()
            .as_ref()
            .and_then(|root| root.join(path).ok())
    }

    fn load(&self, url: &Url) -> Option<Ref<Url, SrcTree>> {
        self.document_map
            .entry(url.clone())
            .or_try_insert_with(|| {
                let src = std::fs::read_to_string(url.path())?;
                let src_tree = SrcTree::new(src);
                std::io::Result::Ok(src_tree)
            })
            .map(|rm| rm.downgrade())
            .ok()
    }

    // fn definition(&self, url: Url, ident: &str) -> Option<Definition> {
    //     let mut visited = HashSet::new();
    //
    //     let mut stack = vec![url];
    //
    //     while let Some(url) = stack.pop() {
    //         if visited.contains(&url) {
    //             continue;
    //         }
    //         if let Some(src_tree) = self.load(&url) {
    //             if let Some(node) = src_tree.definition(ident) {
    //                 let src = src_tree.src();
    //                 let src = node.utf8_text(src.as_bytes()).unwrap().to_string();
    //                 let start = node.start_position();
    //                 let end = node.end_position();
    //                 let range = Range {
    //                     start: Position {
    //                         line: start.row as _,
    //                         character: start.column as _,
    //                     },
    //                     end: Position {
    //                         line: end.row as _,
    //                         character: end.column as _,
    //                     },
    //                 };
    //
    //                 return Some(Definition { src, url, range });
    //             }
    //
    //             for path in src_tree.includes() {
    //                 if let Some(url) = self.url(&path) {
    //                     stack.push(url);
    //                 }
    //             }
    //         }
    //         visited.insert(url);
    //     }
    //
    //     None
    // }
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
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "SScript LSP".to_string(),
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

    async fn goto_declaration(
        &self,
        params: GotoDeclarationParams,
    ) -> Result<Option<GotoDeclarationResponse>> {
        Ok(Some({ GotoDefinitionResponse::Link(Vec::new()) }))
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file opened!")
            .await;
        let _ = self
            .on_change(TextDocumentItem {
                language_id: params.text_document.language_id,
                uri: params.text_document.uri,
                text: params.text_document.text,
                version: params.text_document.version,
            })
            .await;
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "did_change")
            .await;
        let _ = self
            .on_change(TextDocumentItem {
                language_id: format!("sscript"),
                uri: params.text_document.uri,
                text: std::mem::take(&mut params.content_changes[0].text),
                version: params.text_document.version,
            })
            .await;
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file saved!")
            .await;
    }
    async fn did_close(&self, param: DidCloseTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file closed!")
            .await;

        self.document_map.remove(&param.text_document.uri);
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let position = params.text_document_position_params.text_document;
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
