pub use if_std::AsyncWebsocketClient;

mod if_std {

    use crate::asynch::clients::async_client::AsyncClient;
    use crate::asynch::clients::client::Client;
    use crate::asynch::clients::exceptions::XRPLWebsocketException;
    use crate::asynch::clients::websocket_base::{Websocket, WebsocketBase};
    use crate::models::Model;
    use crate::Err;
    use alloc::borrow::Cow;

    use anyhow::Result;

    use em_as_net::client::websocket::{
        ReadResult, WebsocketClient, WebsocketClientIo, WebsocketSendMessageType,
    };
    use rand::rngs::ThreadRng;
    use serde::Serialize;
    use tokio::net;

    /// An async client for interacting with the rippled WebSocket API.
    pub struct AsyncWebsocketClient<'a> {
        pub uri: Cow<'a, str>,
        inner: WebsocketClient<'a, net::TcpStream, ThreadRng>,
    }

    impl<'a> AsyncWebsocketClient<'a> {
        pub fn new(uri: Cow<'a, str>, buffer: &'a mut [u8]) -> Self {
            let ws = WebsocketClient::new(uri.clone(), buffer);
            Self { uri, inner: ws }
        }
    }

    impl<'a, T: Model + Serialize, R> Websocket<'a, T, R> for AsyncWebsocketClient<'a> {}

    impl<'a, T: Model + Serialize, R> WebsocketBase<'a, T, R> for AsyncWebsocketClient<'a> {
        fn is_open(&self) -> bool {
            self.inner.is_open()
        }

        async fn do_open(&'a mut self) -> Result<()> {
            self.inner.connect(None).await;

            Ok(())
        }

        async fn do_close(&'a mut self) -> Result<()> {
            self.inner.close().await;

            Ok(())
        }

        async fn do_write(&'a mut self, request: T) -> Result<()> {
            let request_string = serde_json::to_string(&request).unwrap(); // TODO: Unhandled unwrap
            self.inner
                .write(
                    Cow::from(request_string),
                    Some(WebsocketSendMessageType::Text),
                )
                .await
        }

        async fn do_read(&'a mut self) -> Option<Result<ReadResult<'a>>> {
            self.inner.read().await
        }

        async fn do_request_impl(&'a mut self, _request: T) -> Result<R> {
            todo!()
        }
    }

    impl<'a, T: Model + Serialize, R> AsyncClient<'a, T, R> for AsyncWebsocketClient<'a> {}

    impl<'a, T: Model + Serialize, R> Client<'a, T, R> for AsyncWebsocketClient<'a> {
        async fn request_impl(&'a mut self, request: T) -> Result<R> {
            if !<AsyncWebsocketClient<'a> as WebsocketBase<'_, T, R>>::is_open(self) {
                return Err!(XRPLWebsocketException::NotOpen);
            }

            self.do_request_impl(request).await
        }
    }
}
