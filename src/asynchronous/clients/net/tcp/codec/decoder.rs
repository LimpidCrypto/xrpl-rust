use super::CodecException;
use crate::Err;
use alloc::string::ToString;
use anyhow::Result;
use bytes::BytesMut;

pub trait Decoder {
    type Item;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>>;

    fn decode_eof(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>> {
        match self.decode(buf)? {
            Some(frame) => Ok(Some(frame)),
            None => {
                if buf.is_empty() {
                    Ok(None)
                } else {
                    Err!(CodecException::DecodeError)
                }
            }
        }
    }
}
