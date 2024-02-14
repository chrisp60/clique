#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("clique=debug")
        .finish();

    let server = clique::Server::new();
    Ok(server.run().await?)
}
