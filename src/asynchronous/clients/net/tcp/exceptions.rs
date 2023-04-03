use embedded_io::ErrorKind;
use thiserror_no_std::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum TcpException {
    ReadError,
    ReadableError,
    WriteError,
    WritableError,
    NotConnected,
}

impl embedded_io::Error for TcpException {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}
