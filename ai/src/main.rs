use ai::{run, setup_tracing, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_tracing();

    let config = Config::load()?;
    tracing::info!(?config, "Configuration loaded");

    if let Err(error) = run(config).await {
        tracing::error!(?error, "Unable to start AI microservice");
    }
    Ok(())
}
