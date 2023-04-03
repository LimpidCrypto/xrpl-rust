// use anyhow::Result;
// use crate::models::Model;
// use super::client::Client;
//
// /// Interface for all async network clients to follow.
// pub trait AsyncClient<T: Model, R>: Client<T, R> {
//     async fn request(&self, request: T) -> Result<R> {
//         self._request_impl(request).await
//     }
// }
