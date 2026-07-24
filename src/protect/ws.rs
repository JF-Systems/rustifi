//! Live event subscriptions over the Protect Integration API WebSocket.
//!
//! The streams end (`poll_next` returns `None`) when the console closes the
//! connection or a transport error occurs. The API offers no replay cursor, so
//! events occurring while disconnected are lost; reconnect by calling the
//! subscribe method again:
//!
//! ```no_run
//! # use futures::StreamExt;
//! # async fn run(client: rustifi::protect::ProtectClient) -> rustifi::Result<()> {
//! loop {
//!     let mut stream = client.subscribe_events().await?;
//!     while let Some(msg) = stream.next().await {
//!         match msg {
//!             Ok(event) => println!("{:?}", event),
//!             Err(e) => eprintln!("stream error: {}", e),
//!         }
//!     }
//!     tokio::time::sleep(std::time::Duration::from_secs(5)).await;
//! }
//! # }
//! ```

use crate::error::{Error, Result};
use crate::protect::client::{ProtectClient, PROTECT_BASE_PATH};
use crate::protect::models::{DeviceUpdate, ProtectEvent, WsMessage};
use futures::stream::Stream;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{
    connect_async_tls_with_config, Connector, MaybeTlsStream, WebSocketStream,
};

type WsStream = WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>;

/// Stream of Protect events from `/subscribe/events`.
pub type ProtectEventStream = TypedWsStream<ProtectEvent>;

/// Stream of partial device updates from `/subscribe/devices`.
pub type DeviceUpdateStream = TypedWsStream<DeviceUpdate>;

/// A WebSocket connection yielding typed `{"type": ..., "item": ...}` frames.
///
/// - Text frames are deserialized to `WsMessage<T>`; failures yield
///   [`Error::Parse`] without ending the stream.
/// - Ping/pong/binary frames are skipped (tungstenite answers pings itself).
/// - A close frame or transport error yields [`Error::WebSocket`] and then the
///   stream ends.
pub struct TypedWsStream<T> {
    inner: WsStream,
    // Set after a close frame or transport error; the stream then ends
    // instead of surfacing follow-up frames from the closing handshake.
    terminated: bool,
    // fn() -> T keeps the stream Unpin/Send regardless of T, which is only
    // ever produced by deserialization, never stored.
    _marker: PhantomData<fn() -> T>,
}

impl<T> std::fmt::Debug for TypedWsStream<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypedWsStream").finish_non_exhaustive()
    }
}

impl<T> Stream for TypedWsStream<T>
where
    T: for<'a> serde::Deserialize<'a>,
{
    type Item = Result<WsMessage<T>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.terminated {
            return Poll::Ready(None);
        }
        loop {
            match Pin::new(&mut self.inner).poll_next(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(None) => {
                    self.terminated = true;
                    return Poll::Ready(None);
                }
                Poll::Ready(Some(Ok(Message::Text(text)))) => {
                    let parsed = serde_json::from_str::<WsMessage<T>>(text.as_str())
                        .map_err(|e| Error::Parse(format!("{}\nFrame: {}", e, text)));
                    return Poll::Ready(Some(parsed));
                }
                Poll::Ready(Some(Ok(Message::Close(frame)))) => {
                    self.terminated = true;
                    let reason = frame
                        .map(|f| format!("connection closed: {} {}", f.code, f.reason))
                        .unwrap_or_else(|| "connection closed".to_string());
                    return Poll::Ready(Some(Err(Error::WebSocket(reason))));
                }
                // Ping/pong/binary/raw frames carry no events; keep polling.
                Poll::Ready(Some(Ok(_))) => continue,
                Poll::Ready(Some(Err(e))) => {
                    self.terminated = true;
                    return Poll::Ready(Some(Err(Error::WebSocket(e.to_string()))));
                }
            }
        }
    }
}

async fn connect<T>(client: &ProtectClient, endpoint: &str) -> Result<TypedWsStream<T>> {
    let mut url = url::Url::parse(client.base_url())?;
    let ws_scheme = match url.scheme() {
        "https" | "wss" => "wss",
        _ => "ws",
    };
    url.set_scheme(ws_scheme).map_err(|_| {
        Error::WebSocket(format!("cannot derive ws url from {}", client.base_url()))
    })?;

    let ws_url = format!(
        "{}/{}/{}",
        url.as_str().trim_end_matches('/'),
        PROTECT_BASE_PATH,
        endpoint
    );

    let mut request = ws_url
        .into_client_request()
        .map_err(|e| Error::WebSocket(e.to_string()))?;
    request
        .headers_mut()
        .insert("X-API-Key", client.api_key().parse()?);

    let connector = if client.accepts_invalid_certs() {
        let tls = native_tls::TlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .build()
            .map_err(|e| Error::WebSocket(e.to_string()))?;
        Some(Connector::NativeTls(tls))
    } else {
        None
    };

    let (inner, _response) = connect_async_tls_with_config(request, None, false, connector)
        .await
        .map_err(|e| Error::WebSocket(e.to_string()))?;

    Ok(TypedWsStream {
        inner,
        terminated: false,
        _marker: PhantomData,
    })
}

impl ProtectClient {
    /// Subscribe to live motion, smart-detection, doorbell, and connectivity
    /// events.
    /// Endpoint: WSS /proxy/protect/integration/v1/subscribe/events
    ///
    /// See the [module docs](self) for the reconnect idiom.
    pub async fn subscribe_events(&self) -> Result<ProtectEventStream> {
        connect(self, "subscribe/events").await
    }

    /// Subscribe to live partial device-state updates.
    /// Endpoint: WSS /proxy/protect/integration/v1/subscribe/devices
    ///
    /// See the [module docs](self) for the reconnect idiom.
    pub async fn subscribe_device_updates(&self) -> Result<DeviceUpdateStream> {
        connect(self, "subscribe/devices").await
    }
}
