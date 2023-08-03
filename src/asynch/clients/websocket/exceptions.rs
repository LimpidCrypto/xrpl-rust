use core::fmt::Debug;
use thiserror_no_std::Error;
use tokio_tungstenite::tungstenite;

#[derive(Debug, Error)]
pub enum XRPLWebsocketException {
    #[cfg(feature = "std")]
    #[error("Tungstenite: `{0:?}`")]
    Tungstenite(tungstenite::Error),
}

#[cfg(feature = "std")]
impl From<tungstenite::Error> for XRPLWebsocketException {
    fn from(value: tungstenite::Error) -> Self {
        Self::Tungstenite(value)
    }
}
