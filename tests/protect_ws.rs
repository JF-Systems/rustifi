#![cfg(feature = "protect")]

use futures::{SinkExt, StreamExt};
use rustifi::protect::models::{ProtectEventType, WsAction};
use rustifi::protect::ProtectClient;
use rustifi::Error;
use tokio_tungstenite::tungstenite::Message;

/// Spawn a WebSocket server on an ephemeral port that accepts one connection,
/// sends the given frames, and then closes.
async fn spawn_ws_server(frames: Vec<Message>) -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut ws = tokio_tungstenite::accept_async(stream).await.unwrap();
        for frame in frames {
            ws.send(frame).await.unwrap();
        }
        let _ = ws.close(None).await;
    });

    format!("http://{}", addr)
}

fn event_frame(event_type: &str) -> Message {
    Message::Text(
        format!(
            r#"{{"type":"add","item":{{"id":"e1","modelKey":"event","type":"{}","start":1748517571853,"device":"cam1","smartDetectTypes":[]}}}}"#,
            event_type
        )
        .into(),
    )
}

#[tokio::test]
async fn test_event_stream_decodes_text_frames() {
    let base_url =
        spawn_ws_server(vec![event_frame("motion"), event_frame("smartDetectZone")]).await;

    let client = ProtectClient::new(base_url, "test-key").unwrap();
    let mut stream = client.subscribe_events().await.unwrap();

    let first = stream.next().await.unwrap().unwrap();
    assert_eq!(first.action, WsAction::Add);
    assert_eq!(first.item.event_type, ProtectEventType::Motion);

    let second = stream.next().await.unwrap().unwrap();
    assert_eq!(second.item.event_type, ProtectEventType::SmartDetectZone);
}

#[tokio::test]
async fn test_event_stream_skips_pings_and_ends_on_close() {
    let base_url = spawn_ws_server(vec![
        Message::Ping(vec![1, 2, 3].into()),
        event_frame("doorbell"),
    ])
    .await;

    let client = ProtectClient::new(base_url, "test-key").unwrap();
    let mut stream = client.subscribe_events().await.unwrap();

    // The ping must be skipped, yielding the doorbell event first.
    let first = stream.next().await.unwrap().unwrap();
    assert_eq!(first.item.event_type, ProtectEventType::Doorbell);

    // Server close: a WebSocket error item, then the stream ends.
    match stream.next().await {
        Some(Err(Error::WebSocket(_))) | None => {}
        other => panic!("expected close or end of stream, got {:?}", other.is_some()),
    }
    assert!(stream.next().await.is_none());
}

#[tokio::test]
async fn test_event_stream_surfaces_parse_errors_and_continues() {
    let base_url = spawn_ws_server(vec![
        Message::Text("this is not json".to_string().into()),
        event_frame("motion"),
    ])
    .await;

    let client = ProtectClient::new(base_url, "test-key").unwrap();
    let mut stream = client.subscribe_events().await.unwrap();

    match stream.next().await {
        Some(Err(Error::Parse(_))) => {}
        other => panic!("expected parse error, got ok={:?}", other.is_some()),
    }

    // The stream must survive a malformed frame.
    let next = stream.next().await.unwrap().unwrap();
    assert_eq!(next.item.event_type, ProtectEventType::Motion);
}
