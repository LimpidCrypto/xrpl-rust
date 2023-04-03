use crate::asynchronous::clients::net::tcp::codec::framed::framed_impl::{FramedImpl, RWFrames};
use pin_project_lite::pin_project;

pub(crate) mod framed_impl;

pin_project! {
    pub struct Framed<T, U> {
        #[pin]
        inner: FramedImpl<T, U, RWFrames>
    }
}
