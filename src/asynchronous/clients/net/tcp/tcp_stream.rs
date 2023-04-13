use alloc::borrow::Cow;
use anyhow::Result;
use core::cell::RefCell;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::{Sink, Stream};
use heapless::Vec;

pub trait TcpHandler<'a> {
    async fn connect(&self, url: Cow<'a, str>) -> Result<()>;
}

pub struct TcpStream<T> {
    pub stream: RefCell<Option<T>>,
}

impl<T> TcpStream<T> {
    pub fn new() -> Self {
        Self {
            stream: RefCell::new(None),
        }
    }
}

// impl<Item, T> Sink<Item> for TcpStream<T>
// {
//     type Error = anyhow::Error;
//
//     fn poll_ready(self, cx: &mut Context<'_>) -> Poll<core::result::Result<(), Self::Error>> {
//         todo!()
//     }
//
//     fn start_send(self: Pin<&mut Self>, item: Item) -> core::result::Result<(), Self::Error> {
//         todo!()
//     }
//
//     fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<core::result::Result<(), Self::Error>> {
//         todo!()
//     }
//
//     fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<core::result::Result<(), Self::Error>> {
//         todo!()
//     }
// }

#[cfg(feature = "std")]
mod std_tcp {
    use crate::asynchronous::clients::net::tcp::codec::Codec;
    use crate::asynchronous::clients::net::tcp::{TcpHandler, TcpStream};
    use crate::Err;
    use alloc::borrow::Cow;
    use alloc::string::ToString;
    use anyhow::Result;
    use futures::{Sink, Stream};
    use tokio::net;
    use tokio_util::codec::Framed;

    impl<'a> TcpHandler<'a> for TcpStream<Framed<net::TcpStream, Codec>> {
        async fn connect(&self, url: Cow<'a, str>) -> Result<()> {
            let result = net::TcpStream::connect(&*url).await;

            match result {
                Ok(tcp_stream) => {
                    self.stream
                        .replace(Some(Framed::new(tcp_stream, Codec::new())));
                    Ok(())
                }
                Err(error) => {
                    Err!(error)
                }
            }
        }
    }

    // impl<Item, T: Sink<Item> + Stream> Io for TcpStream<Item, T> {
    //     type Error = TcpException;
    // }
    //
    // impl<Item, T: Sink<Item> + Stream> Read for TcpStream<Item, T> {
    //     async fn read(&mut self, buf: &mut [u8]) -> core::result::Result<usize, Self::Error> {
    //         let tcp_stream_opt: RefMut<Option<net::TcpStream>> = self.stream.borrow_mut();
    //         let tcp_stream_ref = tcp_stream_opt.as_ref();
    //
    //         match tcp_stream_ref {
    //             Some(tcp_stream) => {
    //                 // wait for stream is readable
    //                 match tcp_stream.readable().await {
    //                     Ok(_) => match tcp_stream.try_read(buf) {
    //                         Ok(len) => Ok(len),
    //                         Err(_) => Err(TcpException::ReadError),
    //                     },
    //                     Err(_) => Err(TcpException::ReadableError),
    //                 }
    //             }
    //             None => Err(TcpException::NotConnected),
    //         }
    //     }
    // }
    //
    // impl<Item, T: Sink<Item> + Stream> Write for TcpStream<Item, T> {
    //     async fn write(&mut self, buf: &[u8]) -> core::result::Result<usize, Self::Error> {
    //         let tcp_stream_opt: RefMut<Option<net::TcpStream>> = self.stream.borrow_mut();
    //         let tcp_stream_ref = tcp_stream_opt.as_ref();
    //
    //         match tcp_stream_ref {
    //             Some(tcp_stream) => {
    //                 // wait for stream is writable
    //                 match tcp_stream.writable().await {
    //                     Ok(_) => match tcp_stream.try_write(buf) {
    //                         Ok(len) => Ok(len),
    //                         Err(_) => Err(TcpException::WriteError),
    //                     },
    //                     Err(_) => Err(TcpException::WritableError),
    //                 }
    //             }
    //             None => Err(TcpException::NotConnected),
    //         }
    //     }
    // }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod test_stream {
    use super::*;
    use crate::asynchronous::clients::net::tcp::codec::Codec;
    use embedded_websocket::framer_async::Framer;
    use embedded_websocket::{WebSocketClient, WebSocketOptions};
    use tokio::net;
    use tokio::runtime::Runtime;
    use tokio_util::codec::Framed;

    #[test]
    fn test() {
        async fn main_task() {
            // define an empty `TcpStream`
            let mut stream: TcpStream<Framed<net::TcpStream, Codec>> = TcpStream::new();
            // connect to a host
            // stream.connect("example.com:80".into()).await.unwrap();
            // let mut stream = Framed::new(stream, Codec::new());
            // write bytes to stream
            // let size_written = stream.write("hello world".as_bytes()).await.unwrap();
            //
            // let mut dst = [0; 4096];
            // let size_read = stream.read(&mut dst).await.unwrap();
            //
            // assert_eq!(size_written, 11 as usize);
            // assert_eq!(size_read, 516 as usize);

            let rng = rand::thread_rng();
            let ws = WebSocketClient::new_client(rng);

            let websocket_options = WebSocketOptions {
                path: "/",
                host: "localhost",
                origin: "http://localhost:8080",
                sub_protocols: None,
                additional_headers: None,
            };

            let mut buffer = [0u8; 4096];
            let mut framer = Framer::new(ws);
            framer
                .connect(&mut stream.stream, &mut buffer, &websocket_options)
                .await
                .unwrap();
        }

        let runtime = Runtime::new().unwrap();
        runtime.block_on(main_task())
    }
}
