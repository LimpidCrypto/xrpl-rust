use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{Model, RequestMethod};

/// The server_info command asks the server for a
/// human-readable version of various information about the
/// rippled server being queried.
///
/// See Server Info:
/// `<https://xrpl.org/server_info.html#server_info>`
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfo<'a> {
    /// The unique request id.
    pub id: Option<&'a str>,
    /// The request info.
    #[serde(default = "RequestMethod::server_info")]
    pub command: RequestMethod,
}

impl<'a> Default for ServerInfo<'a> {
    fn default() -> Self {
        ServerInfo {
            id: None,
            command: RequestMethod::ServerInfo,
        }
    }
}

impl<'a> Model for ServerInfo<'a> {}
