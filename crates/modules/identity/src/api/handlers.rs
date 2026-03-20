use axum::{extract::State, Json};
use washco_shared::{AppError, TenantContext};

use super::dto::*;
use super::IdentityState;
use crate::application::RegisterInput;

pub async fn register(
    State(svc): State<IdentityState>,
    Json(body): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = svc
        .register(RegisterInput {
            phone: body.phone,
            business_name: body.business_name,
            owner_name: body.owner_name,
        })
        .await?;

    Ok(Json(user.into()))
}

pub async fn request_otp(
    State(svc): State<IdentityState>,
    Json(body): Json<OtpRequest>,
) -> Result<Json<MessageResponse>, AppError> {
    svc.request_otp(&body.phone).await?;
    Ok(Json(MessageResponse {
        message: "OTP sent".to_string(),
    }))
}

pub async fn verify_otp(
    State(svc): State<IdentityState>,
    Json(body): Json<OtpVerifyRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    let tokens = svc.verify_otp_and_login(&body.phone, &body.code).await?;
    Ok(Json(TokenResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
    }))
}

pub async fn refresh(
    State(svc): State<IdentityState>,
    Json(body): Json<RefreshRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    let tokens = svc.refresh_token(&body.refresh_token)?;
    Ok(Json(TokenResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
    }))
}

pub async fn me(
    State(svc): State<IdentityState>,
    ctx: TenantContext,
) -> Result<Json<UserResponse>, AppError> {
    let user = svc.get_user(ctx.user_id).await?;
    Ok(Json(user.into()))
}
