#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("clique=debug")
        .init();

    let server = clique::Server::new();
    server.run().await
}
