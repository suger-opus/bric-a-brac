use knowledge::{infrastructure::Config, presentation::setup_tracing, run};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_tracing();

    let config = Config::load()?;
    tracing::info!(
        ?config,
        "Starting knowledge microservice with configuration"
    );

    if let Err(error) = run(&config).await {
        tracing::error!(?error, "Unable to start knowledge microservice");
    }
    Ok(())
}
