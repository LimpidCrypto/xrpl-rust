use crate::asynch::clients::client::Client;
use crate::asynch::clients::exceptions::XRPLWebsocketException;
use crate::models::Model;
use crate::Err;
use anyhow::Result;
use em_as_net::client::websocket::ReadResult;
use serde::Serialize;

// A client for interacting with the rippled WebSocket API.
pub trait WebsocketBase<'a, T: Model + Serialize, R>: Client<'a, T, R> {
    fn is_open(&self) -> bool;

    async fn do_open(&'a mut self) -> Result<()>;

    async fn do_close(&'a mut self) -> Result<()>;

    async fn do_write(&'a mut self, request: T) -> Result<()>;

    async fn do_read(&'a mut self) -> Option<Result<ReadResult<'a>>>;

    async fn do_request_impl(&'a mut self, request: T) -> Result<R>;
}

pub trait Websocket<'a, T: Model + Serialize, R>: WebsocketBase<'a, T, R> {
    async fn open(&'a mut self) -> Result<()> {
        if !self.is_open() {
            self.do_open().await
        } else {
            Ok(())
        }
    }

    async fn close(&'a mut self) -> Result<()> {
        if self.is_open() {
            self.do_close().await
        } else {
            Ok(())
        }
    }

    async fn write(&'a mut self, request: T) -> Result<()> {
        if self.is_open() {
            self.do_write(request).await
        } else {
            Err!(XRPLWebsocketException::NotOpen)
        }
    }

    async fn read(&'a mut self) -> Option<Result<ReadResult<'a>>> {
        if self.is_open() {
            self.do_read().await
        } else {
            Some(Err!(XRPLWebsocketException::NotOpen))
        }
    }
}
