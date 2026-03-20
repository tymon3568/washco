use anyhow::Context;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub jwt_expiry_seconds: i64,
    pub jwt_refresh_expiry_seconds: i64,
    pub s3_endpoint: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub s3_region: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .context("PORT must be a valid u16")?,
            database_url: std::env::var("DATABASE_URL").context("DATABASE_URL is required")?,
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            jwt_secret: std::env::var("JWT_SECRET").context("JWT_SECRET is required")?,
            jwt_expiry_seconds: std::env::var("JWT_EXPIRY_SECONDS")
                .unwrap_or_else(|_| "900".to_string())
                .parse()
                .context("JWT_EXPIRY_SECONDS must be a valid i64")?,
            jwt_refresh_expiry_seconds: std::env::var("JWT_REFRESH_EXPIRY_SECONDS")
                .unwrap_or_else(|_| "604800".to_string())
                .parse()
                .context("JWT_REFRESH_EXPIRY_SECONDS must be i64")?,
            s3_endpoint: std::env::var("S3_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:9000".to_string()),
            s3_bucket: std::env::var("S3_BUCKET")
                .unwrap_or_else(|_| "washco".to_string()),
            s3_access_key: std::env::var("S3_ACCESS_KEY")
                .unwrap_or_else(|_| "washco".to_string()),
            s3_secret_key: std::env::var("S3_SECRET_KEY")
                .unwrap_or_else(|_| "washco-secret".to_string()),
            s3_region: std::env::var("S3_REGION")
                .unwrap_or_else(|_| "us-east-1".to_string()),
        })
    }
}
