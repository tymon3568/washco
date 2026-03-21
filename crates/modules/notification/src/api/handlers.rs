use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use washco_shared::{AppError, TenantContext};

use super::dto::*;
use super::NotificationState;
use crate::application::{CreateTemplateInput, SendNotificationInput, UpdateTemplateInput};

pub async fn create_template(
    State(svc): State<NotificationState>,
    ctx: TenantContext,
    Json(body): Json<CreateTemplateRequest>,
) -> Result<(StatusCode, Json<TemplateResponse>), AppError> {
    ctx.require_manager_or_above()?;
    let template = svc
        .create_template(
            ctx.tenant_id,
            CreateTemplateInput {
                template_type: body.template_type,
                channel: body.channel.unwrap_or_else(|| "sms".to_string()),
                subject: body.subject,
                body_template: body.body_template,
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(template.into())))
}

pub async fn list_templates(
    State(svc): State<NotificationState>,
    ctx: TenantContext,
) -> Result<Json<Vec<TemplateResponse>>, AppError> {
    let templates = svc.list_templates(ctx.tenant_id).await?;
    Ok(Json(templates.into_iter().map(Into::into).collect()))
}

pub async fn update_template(
    State(svc): State<NotificationState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateTemplateRequest>,
) -> Result<Json<TemplateResponse>, AppError> {
    ctx.require_manager_or_above()?;
    let template = svc
        .update_template(
            ctx.tenant_id,
            UpdateTemplateInput {
                id,
                template_type: body.template_type,
                channel: body.channel.unwrap_or_else(|| "sms".to_string()),
                subject: body.subject,
                body_template: body.body_template,
                is_active: body.is_active,
            },
        )
        .await?;

    Ok(Json(template.into()))
}

pub async fn delete_template(
    State(svc): State<NotificationState>,
    ctx: TenantContext,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    ctx.require_manager_or_above()?;
    svc.delete_template(ctx.tenant_id, id).await?;
    Ok(Json(serde_json::json!({ "message": "Deleted" })))
}

pub async fn send_notification(
    State(svc): State<NotificationState>,
    ctx: TenantContext,
    Json(body): Json<SendNotificationRequest>,
) -> Result<(StatusCode, Json<NotificationResponse>), AppError> {
    let notification = svc
        .send_notification(
            ctx.tenant_id,
            SendNotificationInput {
                recipient_phone: body.recipient_phone,
                template_type: body.template_type,
                channel: body.channel.unwrap_or_else(|| "sms".to_string()),
                payload: body.payload.unwrap_or(serde_json::json!({})),
            },
        )
        .await?;

    Ok((StatusCode::CREATED, Json(notification.into())))
}

pub async fn list_notifications(
    State(svc): State<NotificationState>,
    ctx: TenantContext,
    Query(params): Query<ListNotificationsQuery>,
) -> Result<Json<Vec<NotificationResponse>>, AppError> {
    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);
    let notifications = svc.list_notifications(ctx.tenant_id, limit, offset).await?;
    Ok(Json(notifications.into_iter().map(Into::into).collect()))
}
