use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod outbox;
mod router;
mod state;
mod uploads;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::AppConfig::from_env()?;

    // --outbox flag: run only the outbox processor
    if std::env::args().any(|a| a == "--outbox") {
        let pool = sqlx::PgPool::connect(&config.database_url).await?;
        tracing::info!("Starting outbox processor only");
        return outbox::run(pool).await;
    }

    let state = state::AppState::new(&config).await?;

    // Spawn outbox processor as background task
    let outbox_pool = state.db.clone();
    tokio::spawn(async move {
        if let Err(e) = outbox::run(outbox_pool).await {
            tracing::error!("Outbox processor crashed: {e}");
        }
    });

    let app = router::build(state);

    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("WashCo API starting on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
