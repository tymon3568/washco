use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use washco_shared::{AppError, TenantContext};

use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct PresignRequest {
    pub filename: String,
    pub content_type: String,
    pub folder: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PresignResponse {
    pub upload_url: String,
    pub object_key: String,
    pub public_url: String,
}

pub async fn presign_upload(
    State(state): State<AppState>,
    ctx: TenantContext,
    Json(body): Json<PresignRequest>,
) -> Result<(StatusCode, Json<PresignResponse>), AppError> {
    let ext = body.filename.rsplit('.').next().unwrap_or("bin");

    let folder = body.folder.as_deref().unwrap_or("uploads");
    let object_key = format!("{}/{}/{}.{}", ctx.tenant_id, folder, Uuid::now_v7(), ext);

    let presigned = state
        .s3
        .put_object()
        .bucket(&state.s3_bucket)
        .key(&object_key)
        .content_type(&body.content_type)
        .presigned(
            aws_sdk_s3::presigning::PresigningConfig::builder()
                .expires_in(std::time::Duration::from_secs(300))
                .build()
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Presign config: {e}")))?,
        )
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Presign error: {e}")))?;

    let upload_url = presigned.uri().to_string();
    let public_url = format!("/api/v1/files/{}", object_key);

    Ok((
        StatusCode::CREATED,
        Json(PresignResponse {
            upload_url,
            object_key,
            public_url,
        }),
    ))
}

pub async fn get_file(
    State(state): State<AppState>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> Result<axum::response::Redirect, AppError> {
    let presigned = state
        .s3
        .get_object()
        .bucket(&state.s3_bucket)
        .key(&key)
        .presigned(
            aws_sdk_s3::presigning::PresigningConfig::builder()
                .expires_in(std::time::Duration::from_secs(3600))
                .build()
                .map_err(|e| AppError::Internal(anyhow::anyhow!("Presign config: {e}")))?,
        )
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Presign error: {e}")))?;

    Ok(axum::response::Redirect::temporary(presigned.uri()))
}
