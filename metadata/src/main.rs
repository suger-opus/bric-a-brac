use metadata::{
    infrastructure::config::Config, presentation::tracing::setup as setup_tracing, run,
};

// TODO: create a struct for labels, formatted labels and colors
// TODO: big thinking: formatted_labels should generated ?

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
