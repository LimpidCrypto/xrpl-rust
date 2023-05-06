use crate::asynch::clients::client::Client;
use crate::models::Model;
use anyhow::Result;

// A client for interacting with the rippled WebSocket API.
pub trait WebsocketBase<'a, T: Model, R>: Client<T, R> {
    fn is_open(&self) -> bool;

    async fn _do_open(&self, buffer: &'a mut [u8]) -> Result<()>;

    async fn _do_close(&self) -> Result<()>;

    async fn _handler(&self) -> Result<()>;

    fn _set_up_future(&self) -> Result<()>;

    async fn _do_send_no_future(&self) -> Result<()>;

    async fn _do_send(&self) -> Result<()>;

    async fn _do_pop_future(&self) -> Result<R>;

    async fn _do_request_impl(&self, request: T) -> Result<R>;
}
