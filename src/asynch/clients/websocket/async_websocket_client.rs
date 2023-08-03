use crate::asynch::clients::websocket::XRPLWebsocketException;
use crate::models::Model;
use crate::Err;
use anyhow::Result;
use core::marker::PhantomData;
use core::pin::Pin;
use futures::{Sink, TryStreamExt};
use serde::Serialize;
use tokio_tungstenite::connect_async;
use url::Url;

#[cfg(feature = "std")]
pub use tokio::net::TcpStream;
#[cfg(feature = "std")]
pub use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

#[cfg(feature = "std")]
pub type AsyncWebsocketClientTokio<Status> =
    AsyncWebsocketClient<WebSocketStream<MaybeTlsStream<TcpStream>>, Status>;

pub struct Open;
pub struct Closed;

pub struct AsyncWebsocketClient<T, Status = Closed> {
    inner: T,
    status: PhantomData<Status>,
}

impl<T, Status> AsyncWebsocketClient<T, Status> {
    pub fn is_open(&self) -> bool {
        core::any::type_name::<Status>() == core::any::type_name::<Open>()
    }
}

#[cfg(feature = "std")]
impl WebsocketOpen<WebSocketStream<MaybeTlsStream<TcpStream>>>
    for AsyncWebsocketClient<WebSocketStream<MaybeTlsStream<TcpStream>>, Closed>
{
    async fn open(
        uri: Url,
    ) -> Result<AsyncWebsocketClient<WebSocketStream<MaybeTlsStream<TcpStream>>, Open>> {
        let (websocket_stream, _) = connect_async(uri.clone()).await.unwrap();

        Ok(AsyncWebsocketClient {
            inner: websocket_stream,
            status: PhantomData::<Open>,
        })
    }
}

#[cfg(feature = "std")]
impl WebsocketIO for AsyncWebsocketClient<WebSocketStream<MaybeTlsStream<TcpStream>>, Open> {
    async fn send<Request: Model + Serialize>(&mut self, request: Request) -> Result<()> {
        let request_as_string = serde_json::to_string(&request).unwrap();
        let message = Message::Text(request_as_string);
        match Pin::new(&mut self.inner).start_send(message) {
            Ok(()) => Ok(()),
            Err(ws_error) => Err!(XRPLWebsocketException::Tungstenite(ws_error)),
        }
    }

    async fn on_message(&mut self) -> Result<Option<Message>> {
        // let t = self.inner.next().await;
        match self.inner.try_next().await {
            Ok(message) => Ok(message),
            Err(ws_error) => Err!(XRPLWebsocketException::from(ws_error)),
        }
    }
}

#[cfg(feature = "std")]
impl WebsocketClose<WebSocketStream<MaybeTlsStream<TcpStream>>>
    for AsyncWebsocketClient<WebSocketStream<MaybeTlsStream<TcpStream>>, Open>
{
    async fn close(mut self) -> Result<()> {
        match self.inner.close(None).await {
            Ok(()) => Ok(()),
            Err(ws_error) => Err!(XRPLWebsocketException::from(ws_error)),
        }
    }
}

pub trait WebsocketOpen<T> {
    async fn open(uri: Url) -> Result<AsyncWebsocketClient<T, Open>>;
}

pub trait WebsocketIO {
    async fn send<Request: Model + Serialize>(&mut self, request: Request) -> Result<()>;
    async fn on_message(&mut self) -> Result<Option<Message>>;
}

pub trait WebsocketClose<T> {
    async fn close(self) -> Result<()>;
}
