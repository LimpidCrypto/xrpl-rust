use anyhow::Result;
use bytes::BytesMut;

pub trait Encoder<Item> {
    fn encode(&mut self, data: Item, dst: &mut BytesMut) -> Result<()>;
}
