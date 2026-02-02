use anyhow::Result;
use jsonrpsee::{
    proc_macros::rpc,
    server::ServerBuilder,
    types::{ErrorCode, ErrorObjectOwned},
};
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

#[rpc(server, client, namespace = "play")]
trait CoreApi {
    #[method(name = "add")]
    fn add(&self, a: i64, b: i64) -> Result<i64, ErrorObjectOwned>;

    #[method(name = "sub")]
    fn sub(&self, a: i64, b: i64) -> Result<i64, ErrorObjectOwned>;

    #[method(name = "hello")]
    fn hello(&self, name: Option<String>) -> Result<String, ErrorObjectOwned>;
}

struct CoreServer;

impl CoreApiServer for CoreServer {
    fn add(&self, a: i64, b: i64) -> Result<i64, ErrorObjectOwned> {
        Ok(a + b)
    }

    fn sub(&self, a: i64, b: i64) -> Result<i64, ErrorObjectOwned> {
        if a < b {
            Err(ErrorObjectOwned::owned(
                ErrorCode::InvalidRequest.code(),
                "Value b is greater than a",
                None::<()>,
            ))
        } else {
            Ok(a - b)
        }
    }

    fn hello(&self, name: Option<String>) -> Result<String, ErrorObjectOwned> {
        let who = name.unwrap_or_else(|| "World".to_string());

        Ok(format!("Hello {}! 👋", who))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let server = ServerBuilder::default()
        .build("127.0.0.1:9944".parse::<SocketAddr>()?)
        .await?;

    let handle = server.start(CoreServer.into_rpc());

    println!("🚀 JSON-RPC server listening on http://127.0.0.1:9944");
    println!("Try in terminal:");
    println!("  curl -X POST -H 'Content-Type: application/json' \\");
    println!(
        "    --data '{{\"jsonrpc\":\"2.0\",\"method\":\"play_add\",\"params\":[10,7],\"id\":1}}' \\"
    );
    println!("    http://127.0.0.1:9944");
    println!();
    println!("Or try: play_hello with {{\"params\":[\"Rustacean\"]}}");

    handle.stopped().await;

    Ok(())
}
