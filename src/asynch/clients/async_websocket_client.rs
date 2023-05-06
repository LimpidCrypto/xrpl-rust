pub use if_std::AsyncWebsocketClient;

mod if_std {
    use crate::_serde::HashMap;
    use crate::asynch::clients::async_client::AsyncClient;
    use crate::asynch::clients::client::Client;
    use crate::asynch::clients::exceptions::XRPLWebsocketException;
    use crate::asynch::clients::websocket_base::WebsocketBase;
    use crate::models::Model;
    use crate::Err;
    use alloc::borrow::Cow;
    use alloc::collections::VecDeque;
    use anyhow::Result;
    use core::cell::RefCell;
    use core::task::Context;
    use em_as_net::client::websocket::WebsocketClient;
    
    use rand::rngs::ThreadRng;
    use tokio::net;

    /// An async client for interacting with the rippled WebSocket API.
    pub struct AsyncWebsocketClient<'a, F> {
        pub uri: Cow<'a, str>,
        buffer: RefCell<Option<&'a mut [u8]>>,
        _open_requests: HashMap<&'a str, F>,
        _websocket: RefCell<Option<WebsocketClient<'a, net::TcpStream, ThreadRng>>>,
        _messages: RefCell<Option<VecDeque<Context<'a>>>>, // TODO: `Context` may be the wrong type
    }

    impl<'a, F> AsyncWebsocketClient<'a, F> {
        pub fn new(uri: Cow<'a, str>) -> Self {
            Self {
                uri,
                buffer: RefCell::new(None),
                _open_requests: HashMap::default(),
                _websocket: RefCell::new(None),
                _messages: RefCell::new(None),
            }
        }
    }

    impl<'a, F, T: Model, R> WebsocketBase<'a, T, R> for AsyncWebsocketClient<'a, F> {
        fn is_open(&self) -> bool {
            self._messages.borrow().is_some() && self._websocket.borrow().is_some()
            // TODO: Also check `WebSocketState`
        }

        async fn _do_open(&self, buffer: &'a mut [u8]) -> Result<()> {
            let mut ws = WebsocketClient::new(self.uri.clone(), buffer);
            ws.connect(None);
            self._websocket.replace(Some(ws));
            self._messages.replace(Some(VecDeque::new()));

            Ok(())
        }

        async fn _do_close(&self) -> Result<()> {
            todo!()
        }

        async fn _handler(&self) -> Result<()> {
            todo!()
        }

        fn _set_up_future(&self) -> Result<()> {
            todo!()
        }

        async fn _do_send_no_future(&self) -> Result<()> {
            todo!()
        }

        async fn _do_send(&self) -> Result<()> {
            todo!()
        }

        async fn _do_pop_future(&self) -> Result<R> {
            todo!()
        }

        async fn _do_request_impl(&self, _request: T) -> Result<R> {
            todo!()
        }
    }

    impl<'a, F, T: Model, R> AsyncClient<T, R> for AsyncWebsocketClient<'a, F> {}

    impl<'a, F, T: Model, R> Client<T, R> for AsyncWebsocketClient<'a, F> {
        async fn _request_impl(&self, request: T) -> Result<R> {
            if !<AsyncWebsocketClient<'a, F> as WebsocketBase<'_, T, R>>::is_open(self) {
                return Err!(XRPLWebsocketException::NotOpen);
            }

            self._do_request_impl(request).await
        }
    }
}
