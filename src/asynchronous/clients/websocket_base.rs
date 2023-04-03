// use core::cell::RefCell;
// use anyhow::Result;
// use core::future::Future;
// use core::task::Context;
// use heapless::mpmc::MpMcQueue;
// use embedded_websocket::{WebSocketClient, WebSocketState, WebSocketType};
// use rand::RngCore;
// use crate::_serde::HashMap;
// use crate::asynchronous::clients::client::Client;
// use super::async_client::AsyncClient;
// use crate::models::Model;
// use crate::models::requests::Response;
//
// pub struct WebsocketBase<'a, F: Future, C: RngCore, Z, const N: usize> {
//     _open_requests: HashMap<&'a str, F>,
//     _websocket: Option<WebSocketClient<C>>,
//     _handler_task: Option<Context<'a>>,
//     _messages: RefCell<Option<MpMcQueue<Z, N>>>
// }
//
// // A client for interacting with the rippled WebSocket API.
// // impl<'a, F: Future, C: RngCore, T: Model, R: Response, Z, const N: usize> WebsocketBase<'a, F, C, Z, N> {
// //     fn is_open(&self) -> bool {
// //         if let Some(websocket) = &self._websocket {
// //             self._handler_task.is_some()
// //             && self._messages.as_ptr().is_some()
// //             && websocket.state == WebSocketState::Open
// //         }
// //
// //         false
// //     }
// //
// //     async fn _do_open(&self) -> Result<()> {
// //         self._messages.replace(Some(MpMcQueue::new()));
// //
// //         todo!()
// //     }
// //
// //     async fn _do_close(&self) -> () {
// //         todo!()
// //     }
// //
// //     async fn _handler(&self) -> Result<()> {
// //         todo!()
// //     }
// //
// //     fn _set_up_future(&self) -> Result<()> {
// //         todo!()
// //     }
// //
// //     async fn _do_send_no_future(&self) -> Result<()> {
// //         todo!()
// //     }
// //
// //     async fn _do_send(&self) -> Result<()> {
// //         todo!()
// //     }
// //
// //     async fn _do_pop_future(&self) -> Result<R> {
// //         todo!()
// //     }
// //
// //     async fn _do_request_impl(&self, request: T) -> Result<R> {
// //         todo!()
// //     }
// // }
