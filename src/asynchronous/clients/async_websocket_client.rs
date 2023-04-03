// use anyhow::Result;
// use crate::asynchronous::clients::async_client::AsyncClient;
// use crate::asynchronous::clients::client::Client;
// use crate::asynchronous::clients::exceptions::XRPLWebsocketException;
// use crate::Err;
// use crate::models::Model;
//
// /// An async client for interacting with the rippled WebSocket API.
// pub struct AsyncWebsocketClient {}
//
// impl<T: Model, R> AsyncClient<T, R> for AsyncWebsocketClient {}
//
// impl<T: Model, R> Client<T, R> for AsyncWebsocketClient {
//     async fn _request_impl(&self, request: T) -> Result<R> {
//         if !self.is_open() {
//             Err!(XRPLWebsocketException::NotOpen)
//         }
//
//         self._do_request_impl(request).await
//     }
// }
