use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use washco_server::{config, outbox, router, state};

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
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await?;
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

    let app = router::build(state, &config);

    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("WashCo API starting on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    // Graceful shutdown on SIGTERM/SIGINT
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server shut down gracefully");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => { tracing::info!("Received SIGINT, starting graceful shutdown"); },
        () = terminate => { tracing::info!("Received SIGTERM, starting graceful shutdown"); },
    }
}
