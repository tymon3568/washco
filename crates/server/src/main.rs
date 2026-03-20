use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod router;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::AppConfig::from_env()?;
    let state = state::AppState::new(&config).await?;
    let app = router::build(state);

    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("WashCo API starting on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
