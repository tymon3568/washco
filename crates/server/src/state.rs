use sqlx::PgPool;
use washco_shared::JwtConfig;

use crate::config::AppConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: redis::Client,
    pub jwt: JwtConfig,
}

impl AppState {
    pub async fn new(config: &AppConfig) -> anyhow::Result<Self> {
        let db = PgPool::connect(&config.database_url).await?;
        tracing::info!("Connected to PostgreSQL");

        sqlx::migrate!("../../migrations").run(&db).await?;
        tracing::info!("Migrations applied");

        let redis = redis::Client::open(config.redis_url.as_str())?;
        tracing::info!("Connected to KeyDB");

        let jwt = JwtConfig::new(
            &config.jwt_secret,
            config.jwt_expiry_seconds,
            config.jwt_refresh_expiry_seconds,
        );

        Ok(Self { db, redis, jwt })
    }
}

impl AsRef<JwtConfig> for AppState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}
