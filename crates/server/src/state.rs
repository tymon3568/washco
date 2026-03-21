use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;
use washco_shared::JwtConfig;

use crate::config::AppConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: redis::Client,
    pub jwt: JwtConfig,
    pub s3: aws_sdk_s3::Client,
    pub s3_bucket: String,
}

impl AppState {
    pub async fn new(config: &AppConfig) -> anyhow::Result<Self> {
        let db = PgPoolOptions::new()
            .max_connections(config.db_max_connections)
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(config.db_idle_timeout_seconds))
            .max_lifetime(Duration::from_secs(config.db_max_lifetime_seconds))
            .connect(&config.database_url)
            .await?;
        tracing::info!(
            max_conn = config.db_max_connections,
            "Connected to PostgreSQL"
        );

        sqlx::migrate!("../../migrations").run(&db).await?;
        tracing::info!("Migrations applied");

        let redis = redis::Client::open(config.redis_url.as_str())?;
        tracing::info!("Connected to KeyDB");

        let jwt = JwtConfig::new(
            &config.jwt_secret,
            config.jwt_expiry_seconds,
            config.jwt_refresh_expiry_seconds,
        );

        let s3_creds = aws_sdk_s3::config::Credentials::new(
            &config.s3_access_key,
            &config.s3_secret_key,
            None,
            None,
            "env",
        );
        let s3_config = aws_sdk_s3::config::Builder::new()
            .endpoint_url(&config.s3_endpoint)
            .region(aws_sdk_s3::config::Region::new(config.s3_region.clone()))
            .credentials_provider(s3_creds)
            .force_path_style(true)
            .behavior_version_latest()
            .build();
        let s3 = aws_sdk_s3::Client::from_conf(s3_config);
        tracing::info!("S3 client configured (endpoint: {})", config.s3_endpoint);

        Ok(Self {
            db,
            redis,
            jwt,
            s3,
            s3_bucket: config.s3_bucket.clone(),
        })
    }
}

impl AsRef<JwtConfig> for AppState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}
