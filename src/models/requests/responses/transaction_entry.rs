use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{requests::responses::RequestResponse, Model};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Response<'a> {}

impl<'a> Default for Response<'a> {
    fn default() -> Self {
        todo!()
    }
}

impl<'a> Model for Response<'a> {}

impl<'a> RequestResponse for Response<'a> {
    fn get_response_type(&self) -> ResponseType {
        todo!()
    }
}

impl<'a> Response<'a> {
    pub fn new() -> Self {
        Self {}
    }
}
