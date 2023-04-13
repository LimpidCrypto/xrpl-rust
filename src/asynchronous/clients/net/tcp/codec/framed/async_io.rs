// use core::fmt::{Debug, Formatter};
// use core::marker::PhantomData;
// use core::mem::replace;
// use core::ops::{Deref, DerefMut};
// use anyhow::Result;
// use core::pin::Pin;
// use core::slice;
// use core::task::{Context, Poll};
// use libc::{c_void, iovec};
//
// #[derive(Copy, Clone)]
// #[repr(transparent)]
// pub struct IoSlice<'a> {
//     vec: iovec,
//     _p: PhantomData<&'a [u8]>,
// }
//
// impl<'a> IoSlice<'a> {
//     #[inline]
//     pub fn new(buf: &'a [u8]) -> IoSlice<'a> {
//         IoSlice {
//             vec: iovec { iov_base: buf.as_ptr() as *mut u8 as *mut c_void, iov_len: buf.len() },
//             _p: PhantomData,
//         }
//     }
//
//     #[inline]
//     pub fn advance(&mut self, n: usize) {
//         if self.vec.iov_len < n {
//             panic!("advancing IoSlice beyond its length");
//         }
//
//         unsafe {
//             self.vec.iov_len -= n;
//             self.vec.iov_base = self.vec.iov_base.add(n);
//         }
//     }
//
//     #[inline]
//     pub fn as_slice(&self) -> &[u8] {
//         unsafe { slice::from_raw_parts(self.vec.iov_base as *mut u8, self.vec.iov_len) }
//     }
//
//     #[inline]
//     pub fn advance_slices(bufs: &mut &mut [IoSlice<'a>], n: usize) {
//         // Number of buffers to remove.
//         let mut remove = 0;
//         // Total length of all the to be removed buffers.
//         let mut accumulated_len = 0;
//         for buf in bufs.iter() {
//             if accumulated_len + buf.len() > n {
//                 break;
//             } else {
//                 accumulated_len += buf.len();
//                 remove += 1;
//             }
//         }
//
//         *bufs = &mut replace(bufs, &mut [])[remove..];
//         if bufs.is_empty() {
//             assert_eq!(n, accumulated_len, "advancing io slices beyond their length");
//         } else {
//             bufs[0].advance(n - accumulated_len)
//         }
//     }
// }
//
// unsafe impl<'a> Send for IoSlice<'a> {}
//
// unsafe impl<'a> Sync for IoSlice<'a> {}
//
// impl<'a> Debug for IoSlice<'a> {
//     fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
//         Debug::fmt(self.as_slice(), fmt)
//     }
// }
//
// #[stable(feature = "iovec", since = "1.36.0")]
// impl<'a> Deref for IoSlice<'a> {
//     type Target = [u8];
//
//     #[inline]
//     fn deref(&self) -> &[u8] {
//         self.as_slice()
//     }
// }
//
// #[repr(transparent)]
// pub struct IoSliceMut<'a> {
//     vec: iovec,
//     _p: PhantomData<&'a mut [u8]>,
// }
//
// impl<'a> IoSliceMut<'a> {
//     #[inline]
//     pub fn new(buf: &'a mut [u8]) -> IoSliceMut<'a> {
//         IoSliceMut {
//             vec: iovec { iov_base: buf.as_mut_ptr() as *mut c_void, iov_len: buf.len() },
//             _p: PhantomData,
//         }
//     }
//
//     #[inline]
//     pub fn advance(&mut self, n: usize) {
//         if self.vec.iov_len < n {
//             panic!("advancing IoSliceMut beyond its length");
//         }
//
//         unsafe {
//             self.vec.iov_len -= n;
//             self.vec.iov_base = self.vec.iov_base.add(n);
//         }
//     }
//
//     #[inline]
//     pub fn as_slice(&self) -> &[u8] {
//         unsafe { slice::from_raw_parts(self.vec.iov_base as *mut u8, self.vec.iov_len) }
//     }
//
//     #[inline]
//     pub fn as_mut_slice(&mut self) -> &mut [u8] {
//         unsafe { slice::from_raw_parts_mut(self.vec.iov_base as *mut u8, self.vec.iov_len) }
//     }
//
//     #[inline]
//     pub fn advance_slices(bufs: &mut &mut [IoSliceMut<'a>], n: usize) {
//         // Number of buffers to remove.
//         let mut remove = 0;
//         // Total length of all the to be removed buffers.
//         let mut accumulated_len = 0;
//         for buf in bufs.iter() {
//             if accumulated_len + buf.len() > n {
//                 break;
//             } else {
//                 accumulated_len += buf.len();
//                 remove += 1;
//             }
//         }
//
//         *bufs = &mut replace(bufs, &mut [])[remove..];
//         if bufs.is_empty() {
//             assert_eq!(n, accumulated_len, "advancing io slices beyond their length");
//         } else {
//             bufs[0].advance(n - accumulated_len)
//         }
//     }
// }
//
// unsafe impl<'a> Send for IoSliceMut<'a> {}
//
// unsafe impl<'a> Sync for IoSliceMut<'a> {}
//
// impl<'a> Debug for IoSliceMut<'a> {
//     fn fmt(&self, fmt: &mut Formatter<'_>) -> core::fmt::Result {
//         Debug::fmt(self.as_slice(), fmt)
//     }
// }
//
// impl<'a> Deref for IoSliceMut<'a> {
//     type Target = [u8];
//
//     #[inline]
//     fn deref(&self) -> &[u8] {
//         self.as_slice()
//     }
// }
//
// impl<'a> DerefMut for IoSliceMut<'a> {
//     #[inline]
//     fn deref_mut(&mut self) -> &mut [u8] {
//         self.as_mut_slice()
//     }
// }
//
// pub trait AsyncRead {
//     fn poll_read(
//         self: Pin<&mut Self>,
//         cx: &mut Context<'_>,
//         buf: &mut [u8],
//     ) -> Poll<Result<usize>>;
//
//     fn poll_read_vectored(
//         self: Pin<&mut Self>,
//         cx: &mut Context<'_>,
//         bufs: &mut [IoSliceMut<'_>],
//     ) -> Poll<Result<usize>> {
//         for b in bufs {
//             if !b.is_empty() {
//                 return self.poll_read(cx, b);
//             }
//         }
//
//         self.poll_read(cx, &mut [])
//     }
// }
//
// pub trait AsyncWrite {
//     fn poll_write(
//         self: Pin<&mut Self>,
//         cx: &mut Context<'_>,
//         buf: &[u8],
//     ) -> Poll<Result<usize>>;
//
//     fn poll_write_vectored(
//         self: Pin<&mut Self>,
//         cx: &mut Context<'_>,
//         bufs: &[IoSlice<'_>],
//     ) -> Poll<Result<usize>> {
//         for b in bufs {
//             if !b.is_empty() {
//                 return self.poll_write(cx, b);
//             }
//         }
//
//         self.poll_write(cx, &[])
//     }
//
//     fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>;
//
//     fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>;
// }
//
