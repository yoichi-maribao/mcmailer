use std::convert::Infallible;
use std::sync::Arc;

use axum::{
    extract::State,
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse,
    },
    routing::{get, post},
    Json, Router,
};
use mcmailer_lib::pubsub_message::{parse_pubsub_push_message, PubSubPushBody};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<String>,
}

async fn handle_push(
    State(state): State<Arc<AppState>>,
    Json(body): Json<PubSubPushBody>,
) -> impl IntoResponse {
    match parse_pubsub_push_message(&body.message.data) {
        Ok(decoded_str) => {
            let _ = state.tx.send(decoded_str);
        }
        Err(e) => {
            println!("[pubsub_server] Failed to decode message: {}", e);
        }
    }
    axum::http::StatusCode::OK
}

async fn handle_events(
    State(state): State<Arc<AppState>>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let rx = state.tx.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|result| match result {
        Ok(data) => Some(Ok(Event::default().data(data))),
        Err(_) => None,
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel::<String>(100);
    let state = Arc::new(AppState { tx });

    let app = Router::new()
        .route("/pubsub/push", post(handle_push))
        .route("/events", get(handle_events))
        .with_state(state);

    let port = std::env::var("PUBSUB_PORT").unwrap_or_else(|_| "8090".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("[pubsub_server] Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
