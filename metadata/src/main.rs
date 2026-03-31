use metadata::{run, setup_tracing, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_tracing();

    let config = Config::load()?;
    tracing::debug!(?config, "Configuration loaded");

    if let Err(error) = run(&config).await {
        tracing::error!(?error, "Unable to start metadata microservice");
    }
    Ok(())
}
