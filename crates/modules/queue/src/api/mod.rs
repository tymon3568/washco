use std::sync::Arc;

use axum::{
    Router,
    extract::{
        Path, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
    routing::{get, post, put},
};
use sqlx::PgPool;
use tokio::sync::broadcast::error::RecvError;
use uuid::Uuid;
use washco_shared::JwtConfig;

use crate::application::QueueService;
use crate::infra::PgQueueRepository;

pub mod dto;
mod handlers;
pub mod ws;

type Service = QueueService<PgQueueRepository>;

#[derive(Clone)]
pub struct QueueState {
    service: Arc<Service>,
    jwt: JwtConfig,
    pub broadcast: ws::QueueBroadcast,
}

impl std::ops::Deref for QueueState {
    type Target = Service;
    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl AsRef<JwtConfig> for QueueState {
    fn as_ref(&self) -> &JwtConfig {
        &self.jwt
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<QueueState>,
    Path(location_id): Path<Uuid>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, location_id))
}

async fn handle_socket(mut socket: WebSocket, state: QueueState, location_id: Uuid) {
    let mut rx = state.broadcast.subscribe(location_id).await;

    loop {
        tokio::select! {
            msg = rx.recv() => {
                match msg {
                    Ok(event) => {
                        if socket.send(Message::Text(event.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(RecvError::Lagged(_)) => continue,
                    Err(_) => break,
                }
            }
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(Message::Ping(data))) => {
                        if socket.send(Message::Pong(data)).await.is_err() {
                            break;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn routes(pool: PgPool, jwt: JwtConfig) -> Router {
    let repo = PgQueueRepository::new(pool);
    let service = Arc::new(QueueService::new(repo));
    let broadcast = ws::QueueBroadcast::new();

    let state = QueueState {
        service,
        jwt,
        broadcast,
    };

    Router::new()
        .route("/locations/{location_id}", get(handlers::get_queue))
        .route("/locations/{location_id}/join", post(handlers::join))
        .route("/{id}/advance", put(handlers::advance))
        .route("/{id}/complete", put(handlers::complete))
        .route("/{id}/cancel", put(handlers::cancel))
        .route("/ws/{location_id}", get(ws_handler))
        .with_state(state)
}
