use knowledge::{config::Config, run, setup_tracing};

// TODO for Knowledge microservice:
// - Improve error handling
// - Improve logs
// - (tests)

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
