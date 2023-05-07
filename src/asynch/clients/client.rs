use crate::models::Model;
use anyhow::Result;
use serde::Serialize;

/// Interface for all network clients to follow.
// TODO: `T` should implement a trait `Request`
// TODO: `R` should implement a trait `Response`
pub trait Client<'a, T: Model + Serialize, R> {
    async fn request_impl(&'a mut self, request: T) -> Result<R>;
}
