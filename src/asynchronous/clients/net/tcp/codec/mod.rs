use anyhow::Result;
use bytes::{BufMut, BytesMut};

mod decoder;
mod encoder;
mod exceptions;
mod framed;

pub use decoder::Decoder;
pub use encoder::Encoder;
pub use exceptions::CodecException;
// pub use framed;

pub struct Codec(());

impl Codec {
    pub fn new() -> Self {
        Self(())
    }
}

impl<'a> Encoder<&'a [u8]> for Codec {
    fn encode(&mut self, data: &'a [u8], buf: &mut BytesMut) -> Result<()> {
        buf.reserve(data.len());
        buf.put(data);
        Ok(())
    }
}

impl Decoder for Codec {
    type Item = BytesMut;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>> {
        if !buf.is_empty() {
            let len = buf.len();
            Ok(Some(buf.split_to(len)))
        } else {
            Ok(None)
        }
    }
}
