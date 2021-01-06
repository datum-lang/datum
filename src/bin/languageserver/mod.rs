use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct CharjServer {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for CharjServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult::default())
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::Info, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

pub fn start_server() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();

        let (service, messages) = LspService::new(|client| CharjServer { client });

        Server::new(stdin, stdout)
            .interleave(messages)
            .serve(service)
            .await;
    });
    std::process::exit(1);
}
