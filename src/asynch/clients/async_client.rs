use super::client::Client;
use crate::models::Model;
use anyhow::Result;
use serde::Serialize;

/// Interface for all async network clients to follow.
pub trait AsyncClient<'a, T: Model + Serialize, R>: Client<'a, T, R> {
    async fn request(&'a mut self, request: T) -> Result<R> {
        self.request_impl(request).await
    }
}
