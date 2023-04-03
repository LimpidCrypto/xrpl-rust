use alloc::borrow::Cow;
use anyhow::Result;
use core::cell::RefCell;

pub trait TcpHandler<'a> {
    async fn connect(&self, url: Cow<'a, str>) -> Result<()>;
}

pub struct TcpStream<T> {
    stream: RefCell<Option<T>>,
}

impl<T> TcpStream<T> {
    pub fn new() -> Self {
        Self {
            stream: RefCell::new(None),
        }
    }
}

#[cfg(feature = "std")]
mod std_tcp {
    use crate::asynchronous::clients::net::tcp::exceptions::TcpException;
    use crate::asynchronous::clients::net::tcp::{TcpHandler, TcpStream};
    use crate::Err;
    use alloc::borrow::Cow;
    use alloc::string::ToString;
    use anyhow::Result;
    use core::cell::RefMut;
    use embedded_io::asynch::{Read, Write};
    use embedded_io::Io;
    use tokio::net;

    impl<'a> TcpHandler<'a> for TcpStream<net::TcpStream> {
        async fn connect(&self, url: Cow<'a, str>) -> Result<()> {
            let result = net::TcpStream::connect(&*url).await;

            match result {
                Ok(tcp_stream) => {
                    self.stream.replace(Some(tcp_stream));
                    Ok(())
                }
                Err(error) => {
                    Err!(error)
                }
            }
        }
    }

    impl Io for TcpStream<net::TcpStream> {
        type Error = TcpException;
    }

    impl Read for TcpStream<net::TcpStream> {
        async fn read(&mut self, buf: &mut [u8]) -> core::result::Result<usize, Self::Error> {
            let tcp_stream_opt: RefMut<Option<net::TcpStream>> = self.stream.borrow_mut();
            let tcp_stream_ref = tcp_stream_opt.as_ref();

            match tcp_stream_ref {
                Some(tcp_stream) => {
                    // wait for stream is readable
                    match tcp_stream.readable().await {
                        Ok(_) => match tcp_stream.try_read(buf) {
                            Ok(len) => Ok(len),
                            Err(_) => Err(TcpException::ReadError),
                        },
                        Err(_) => Err(TcpException::ReadableError),
                    }
                }
                None => Err(TcpException::NotConnected),
            }
        }
    }

    impl Write for TcpStream<net::TcpStream> {
        async fn write(&mut self, buf: &[u8]) -> core::result::Result<usize, Self::Error> {
            let tcp_stream_opt: RefMut<Option<net::TcpStream>> = self.stream.borrow_mut();
            let tcp_stream_ref = tcp_stream_opt.as_ref();

            match tcp_stream_ref {
                Some(tcp_stream) => {
                    // wait for stream is writable
                    match tcp_stream.writable().await {
                        Ok(_) => match tcp_stream.try_write(buf) {
                            Ok(len) => Ok(len),
                            Err(_) => Err(TcpException::WriteError),
                        },
                        Err(_) => Err(TcpException::WritableError),
                    }
                }
                None => Err(TcpException::NotConnected),
            }
        }
    }
}

#[cfg(test)]
#[cfg(feature = "std")]
mod test_stream {
    use super::*;
    use crate::asynchronous::clients::net::tcp::codec::{Codec, Framed};
    use embedded_io::asynch::{Read, Write};
    use embedded_websocket::framer_async::Framer;
    use embedded_websocket::{WebSocketClient, WebSocketOptions};
    use tokio::runtime::Runtime;

    #[test]
    fn test() {
        async fn main_task() {
            // define an empty `TcpStream`
            let mut stream = TcpStream::new();
            // connect to a host
            stream.connect("example.com:80".into()).await.unwrap();
            // let mut stream = Framed::new(stream, Codec::new());
            // write bytes to stream
            let size_written = stream.write("hello world".as_bytes()).await.unwrap();

            let mut dst = [0; 4096];
            let size_read = stream.read(&mut dst).await.unwrap();

            assert_eq!(size_written, 11 as usize);
            assert_eq!(size_read, 516 as usize);

            // let rng = rand::thread_rng();
            // let ws = WebSocketClient::new_client(rng);
            //
            // let websocket_options = WebSocketOptions {
            //     path: "/",
            //     host: "localhost",
            //     origin: "http://localhost:8080",
            //     sub_protocols: None,
            //     additional_headers: None,
            // };
            //
            // let mut buffer = [0u8; 4096];
            // let mut framer = Framer::new(ws);
            // framer.connect(&mut stream, &mut buffer, &websocket_options).await?;
        }

        let runtime = Runtime::new().unwrap();
        runtime.block_on(main_task())
    }
}
