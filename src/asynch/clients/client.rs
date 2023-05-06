use crate::models::Model;
use anyhow::Result;

/// Interface for all network clients to follow.
// TODO: `T` should implement a trait `Request`
// TODO: `R` should implement a trait `Response`
pub trait Client<T: Model, R> {
    async fn _request_impl(&self, request: T) -> Result<R>;
}
