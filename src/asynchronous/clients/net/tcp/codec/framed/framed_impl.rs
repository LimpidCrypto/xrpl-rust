// //! A no_std implementation of https://github.com/tokio-rs/tokio/blob/master/tokio-util/src/codec/framed_impl.rs
//
// use crate::asynchronous::clients::net::tcp::codec::{Codec, CodecException, Decoder, Encoder};
// use anyhow::Result;
// use bytes::{Buf, BufMut, BytesMut};
// use core::borrow::{Borrow, BorrowMut};
// use core::future::Future;
// use core::mem::MaybeUninit;
// use core::pin::Pin;
// use core::task::{Context, Poll};
// use embedded_io::asynch::{Read, Write};
// use embedded_io::Io;
// use futures::{ready, Sink, Stream};
// use pin_project_lite::pin_project;
// use super::exceptions::FramedException;
// use crate::Err;
//
// const INITIAL_CAPACITY: usize = 8 * 1024;
// const BACKPRESSURE_BOUNDARY: usize = INITIAL_CAPACITY;
//
// pin_project! {
//     #[derive(Debug)]
//     pub(crate) struct FramedImpl<T, C, State> {
//         #[pin]
//         pub(crate) inner: T,
//         pub(crate) state: State,
//         pub(crate) codec: C,
//     }
// }
//
// #[derive(Debug)]
// pub(crate) struct ReadFrame {
//     pub(crate) eof: bool,
//     pub(crate) is_readable: bool,
//     pub(crate) buffer: BytesMut,
//     pub(crate) has_errored: bool,
// }
//
// pub(crate) struct WriteFrame {
//     pub(crate) buffer: BytesMut,
// }
//
// #[derive(Default)]
// pub(crate) struct RWFrames {
//     pub(crate) read: ReadFrame,
//     pub(crate) write: WriteFrame,
// }
//
// impl Default for ReadFrame {
//     fn default() -> Self {
//         Self {
//             eof: false,
//             is_readable: false,
//             buffer: BytesMut::with_capacity(INITIAL_CAPACITY),
//             has_errored: false,
//         }
//     }
// }
//
// impl Default for WriteFrame {
//     fn default() -> Self {
//         Self {
//             buffer: BytesMut::with_capacity(INITIAL_CAPACITY),
//         }
//     }
// }
//
// impl From<BytesMut> for ReadFrame {
//     fn from(mut buffer: BytesMut) -> Self {
//         let size = buffer.capacity();
//         if size < INITIAL_CAPACITY {
//             buffer.reserve(INITIAL_CAPACITY - size);
//         }
//
//         Self {
//             buffer,
//             is_readable: size > 0,
//             eof: false,
//             has_errored: false,
//         }
//     }
// }
//
// impl From<BytesMut> for WriteFrame {
//     fn from(mut buffer: BytesMut) -> Self {
//         let size = buffer.capacity();
//         if size < INITIAL_CAPACITY {
//             buffer.reserve(INITIAL_CAPACITY - size);
//         }
//
//         Self { buffer }
//     }
// }
//
// impl Borrow<ReadFrame> for RWFrames {
//     fn borrow(&self) -> &ReadFrame {
//         &self.read
//     }
// }
// impl BorrowMut<ReadFrame> for RWFrames {
//     fn borrow_mut(&mut self) -> &mut ReadFrame {
//         &mut self.read
//     }
// }
// impl Borrow<WriteFrame> for RWFrames {
//     fn borrow(&self) -> &WriteFrame {
//         &self.write
//     }
// }
// impl BorrowMut<WriteFrame> for RWFrames {
//     fn borrow_mut(&mut self) -> &mut WriteFrame {
//         &mut self.write
//     }
// }
//
// // impl<T, C> Io for Framed<T, C>
// // where
// //     T: Read + Write,
// //     C: for<'a>Encoder<&'a [u8]> + Decoder
// // {
// //     type Error = CodecException;
// // }
// //
// // impl<T, C> Read for FramedImpl<T, C>
// // where
// //     T: Read + Write,
// //     C: for<'a>Encoder<&'a [u8]> + Decoder
// // {
// //     async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
// //         let read = self.inner.read(buf).await;
// //         let mut buf_mut = BytesMut::from(&*buf);
// //         let mut codec = Codec(());
// //         match codec.decode(&mut buf_mut) {
// //             Ok(buf_mut) => {
// //                 match buf_mut {
// //                     Some(mut buf_mut) => {
// //                         buf.clone_from_slice(buf_mut.as_mut());
// //                         match read {
// //                             Ok(r) => {
// //                                 Ok(r)
// //                             }
// //                             Err(_) => {
// //                                 Err(CodecException::ReadError)
// //                             }
// //                         }
// //                     },
// //                     None => {
// //                         Err(CodecException::ReadEmptyError)
// //                     }
// //                 }
// //             }
// //             Err(_) => {
// //                 Err(CodecException::DecodeError)
// //             }
// //         }
// //     }
// // }
// //
// // impl<T, C> Write for FramedImpl<T, C>
// // where
// //     T: Read + Write,
// //     C: for<'a>Encoder<&'a [u8]> + Decoder
// // {
// //     async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
// //         let mut codec = Codec(());
// //         let mut dst = BytesMut::new();
// //         match codec.encode(buf, &mut dst) {
// //             Ok(_) => {
// //                 let buffer = dst.as_mut();
// //                 match self.inner.write(buffer).await {
// //                     Ok(len) => {Ok(len)}
// //                     Err(_) => {
// //                         Err(CodecException::WriteError)
// //                     }
// //                 }
// //             }
// //             Err(_) => {
// //                 Err(CodecException::EncodeError)
// //             }
// //         }
// //
// //     }
// // }
//
// impl<T, C, R> Stream for FramedImpl<T, C, R>
// where
//     T: Read,
//     C: Decoder,
//     R: BorrowMut<ReadFrame>,
// {
//     type Item = Result<C::Item>;
//
//     async fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
//         let mut pinned = self.project();
//         let state: &mut ReadFrame = pinned.state.borrow_mut();
//
//         loop {
//             if state.has_errored {
//                 state.is_readable = false;
//                 state.has_errored = false;
//                 return Poll::Ready(None);
//             }
//
//             if state.is_readable {
//                 if state.eof {
//                     let frame = pinned.codec.decode_eof(&mut state.buffer).map_err(|err| {
//                         state.has_errored = true;
//                         err
//                     })?;
//                     if frame.is_none() {
//                         state.is_readable = false;
//                     }
//                     return Poll::Ready(frame.map(Ok));
//                 }
//
//                 if let Some(frame) = pinned.codec.decode(&mut state.buffer).map_err(|op| {
//                     state.has_errored = true;
//                     op
//                 })? {
//                     return Poll::Ready(Some(Ok(frame)));
//                 }
//                 state.is_readable = false;
//             }
//             state.buffer.reserve(1);
//             let bytect = match poll_read_buf(pinned.inner.as_mut(), &mut state.buffer).await.map_err(
//                 |err| {
//                     state.has_errored = true;
//                     err
//                 },
//             )? {
//                 Poll::Ready(ct) => ct,
//                 Poll::Pending => return Poll::Pending,
//             };
//             if bytect == 0 {
//                 if state.eof {
//                     return Poll::Ready(None);
//                 }
//                 state.eof = true;
//             } else {
//                 state.eof = false;
//             }
//
//             state.is_readable = true;
//         }
//     }
// }
//
// impl<T, I, U, W> Sink<I> for FramedImpl<T, U, W>
// where
//     T: Write,
//     U: Encoder<I>,
//     W: BorrowMut<WriteFrame>,
// {
//     type Error = anyhow::Error;
//
//     async fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         if self.state.borrow().buffer.len() >= BACKPRESSURE_BOUNDARY {
//             self.as_mut().poll_flush(cx)
//         } else {
//             Poll::Ready(Ok(()))
//         }
//     }
//
//     async fn start_send(self: Pin<&mut Self>, item: I) -> Result<(), Self::Error> {
//         let pinned = self.project();
//         pinned
//             .codec
//             .encode(item, &mut pinned.state.borrow_mut().buffer)?;
//         Ok(())
//     }
//
//     async fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         trace!("flushing framed transport");
//         let mut pinned = self.project();
//
//         while !pinned.state.borrow_mut().buffer.is_empty() {
//             let WriteFrame { buffer, .. } = pinned.state.borrow_mut();
//             trace!(remaining = buffer.len(), "writing;");
//
//             let n = ready!(poll_write_buf(pinned.inner.as_mut(), cx, buffer))?;
//
//             if n == 0 {
//                 return Poll::Ready(Err!(FramedException::WriteToTransport));
//             }
//         }
//
//         // Try flushing the underlying IO
//         ready!(pinned.inner.poll_flush(cx))?;
//
//         trace!("framed transport flushed");
//         Poll::Ready(Ok(()))
//     }
//
//     async fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         ready!(self.as_mut().poll_flush(cx))?;
//         ready!(self.project().inner.poll_shutdown(cx))?;
//
//         Poll::Ready(Ok(()))
//     }
// }
//
// pub async fn poll_read_buf<T: Read, B: BufMut>(
//     io: Pin<&mut T>,
//     buf: &mut B,
// ) -> Poll<Result<usize>> {
//     if !buf.has_remaining_mut() {
//         return Poll::Ready(Ok(0));
//     }
//
//     let n = {
//         let dst = buf.chunk_mut();
//
//         let dst = unsafe { &mut *(dst as *mut _ as *mut [MaybeUninit<u8>]) };
//         let slice = &dst[..0];
//         let ptr = unsafe { &*(slice as *const [MaybeUninit<u8>] as *const [u8]) }.as_ptr();
//
//         ready!(io.read(&mut buf).await?);
//         ptr.len()
//     };
//
//     unsafe {
//         buf.advance_mut(n);
//     }
//
//     Poll::Ready(Ok(n))
// }
//
// pub async fn poll_write_buf<T: Write, B: Buf>(
//     io: Pin<&mut T>,
//     buf: &mut B,
// ) -> Poll<Result<usize>> {
//     const MAX_BUFS: usize = 64;
//
//     if !buf.has_remaining() {
//         return Poll::Ready(Ok(0));
//     }
//
//     let n = ready!(io.write(buf.chunk()).await)?;
//
//     buf.advance(n);
//
//     Poll::Ready(Ok(n))
// }
