use embedded_io::ErrorKind;
use thiserror_no_std::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CodecException {
    #[error("Unable to decode bytes from stream. Bytes remain on stream")]
    DecodeError,
    #[error("Unable to encode bytes")]
    EncodeError,
    #[error("Error occurred during decoding read bytes")]
    ReadError,
    #[error("No bytes to read")]
    ReadEmptyError,
    #[error("Error occurred during encoding bytes to write")]
    WriteError,
}

impl embedded_io::Error for CodecException {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}
