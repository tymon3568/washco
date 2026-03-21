use sqlx::PgPool;

/// Outbox processor that polls for pending events and notifications.
/// Runs as a background task in the server process, or standalone via --outbox flag.
pub async fn run(pool: PgPool) -> anyhow::Result<()> {
    tracing::info!("Outbox processor started");

    loop {
        if let Err(e) = process_outbox_events(&pool).await {
            tracing::error!("Outbox event processing error: {e}");
        }

        if let Err(e) = process_pending_notifications(&pool).await {
            tracing::error!("Notification processing error: {e}");
        }

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

async fn process_outbox_events(pool: &PgPool) -> anyhow::Result<()> {
    let rows = sqlx::query(
        r#"SELECT id, event_type, payload
           FROM outbox_events
           WHERE processed_at IS NULL AND retry_count < 5
           ORDER BY created_at ASC
           LIMIT 50"#,
    )
    .fetch_all(pool)
    .await?;

    for row in rows {
        let id: uuid::Uuid = sqlx::Row::get(&row, "id");
        let event_type: String = sqlx::Row::get(&row, "event_type");
        let payload: serde_json::Value = sqlx::Row::get(&row, "payload");

        tracing::info!(event_type = %event_type, id = %id, "Processing outbox event");

        match dispatch_event(&event_type, &payload).await {
            Ok(()) => {
                sqlx::query("UPDATE outbox_events SET processed_at = now() WHERE id = $1")
                    .bind(id)
                    .execute(pool)
                    .await?;
            }
            Err(e) => {
                tracing::warn!(id = %id, error = %e, "Outbox event failed, incrementing retry");
                sqlx::query(
                    "UPDATE outbox_events SET retry_count = retry_count + 1, error = $2 WHERE id = $1",
                )
                .bind(id)
                .bind(e.to_string())
                .execute(pool)
                .await?;
            }
        }
    }

    Ok(())
}

async fn process_pending_notifications(pool: &PgPool) -> anyhow::Result<()> {
    let rows = sqlx::query(
        r#"SELECT id, recipient_phone, channel, rendered_body
           FROM notifications
           WHERE status = 'pending'
           ORDER BY created_at ASC
           LIMIT 50"#,
    )
    .fetch_all(pool)
    .await?;

    for row in rows {
        let id: uuid::Uuid = sqlx::Row::get(&row, "id");
        let phone: String = sqlx::Row::get(&row, "recipient_phone");
        let channel: String = sqlx::Row::get(&row, "channel");
        let body: Option<String> = sqlx::Row::get(&row, "rendered_body");

        match send_notification(&phone, &channel, body.as_deref()).await {
            Ok(()) => {
                sqlx::query(
                    "UPDATE notifications SET status = 'sent', sent_at = now() WHERE id = $1",
                )
                .bind(id)
                .execute(pool)
                .await?;
                tracing::info!(id = %id, phone = %phone, channel = %channel, "Notification sent");
            }
            Err(e) => {
                sqlx::query("UPDATE notifications SET status = 'failed', error = $2 WHERE id = $1")
                    .bind(id)
                    .bind(e.to_string())
                    .execute(pool)
                    .await?;
                tracing::warn!(id = %id, error = %e, "Notification failed");
            }
        }
    }

    Ok(())
}

async fn dispatch_event(event_type: &str, payload: &serde_json::Value) -> anyhow::Result<()> {
    // Route events to appropriate handlers
    match event_type {
        "booking.confirmed" | "booking.cancelled" => {
            tracing::info!(
                event_type,
                "Booking event processed (would trigger notification)"
            );
        }
        "queue.completed" => {
            tracing::info!(event_type, "Queue completion event processed");
        }
        "review.submitted" => {
            tracing::info!(event_type, "Review event processed");
        }
        _ => {
            tracing::debug!(event_type, payload = %payload, "Unknown event type, skipping");
        }
    }
    Ok(())
}

/// Send notification via appropriate channel.
/// In dev mode, logs the message. In production, integrate real SMS/push providers.
async fn send_notification(phone: &str, channel: &str, body: Option<&str>) -> anyhow::Result<()> {
    let body = body.unwrap_or("(no body)");

    match channel {
        "sms" => {
            // Production: integrate with SMS provider (Twilio, AWS SNS, etc.)
            // For now: log the SMS that would be sent
            if let Ok(sms_api_url) = std::env::var("SMS_API_URL") {
                // Real SMS sending via HTTP API
                let client = reqwest::Client::new();
                let resp = client
                    .post(&sms_api_url)
                    .json(&serde_json::json!({
                        "to": phone,
                        "message": body,
                    }))
                    .send()
                    .await?;

                if !resp.status().is_success() {
                    anyhow::bail!("SMS API returned {}", resp.status());
                }
                tracing::info!(phone, "SMS sent via provider");
            } else {
                tracing::info!(phone, body, "[DEV] SMS would be sent");
            }
        }
        "push" => {
            // Production: integrate with FCM/APNs
            tracing::info!(phone, body, "[DEV] Push notification would be sent");
        }
        _ => {
            tracing::warn!(channel, "Unknown notification channel");
        }
    }

    Ok(())
}
