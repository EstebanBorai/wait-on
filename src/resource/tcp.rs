use std::net::{IpAddr, SocketAddr};
use std::pin::Pin;
use std::task::{Context, Poll};

use anyhow::{Error, Result};
use pin_project::pin_project;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, ReadBuf};
use tokio::net::{TcpListener, TcpStream};

use crate::{WaitOptions, Waitable};

/// Listens on a specific IP Address and Port using TCP protocol
pub struct TcpWaiter {
    pub addr: IpAddr,
    pub port: u16,
}

impl TcpWaiter {
    pub fn new(addr: IpAddr, port: u16) -> Self {
        Self { addr, port }
    }

    pub fn socket(&self) -> SocketAddr {
        SocketAddr::new(self.addr, self.port)
    }
}

impl Waitable for TcpWaiter {
    async fn wait(self, _: WaitOptions) -> Result<()> {
        let tcp_listener = TcpListener::bind(self.socket()).await?;
        let (socket, _) = tcp_listener.accept().await?;
        let mut socket = PacketExtractor::<8>::read(socket).await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    // socket closed
                    return;
                }
            }
        })
        .await
        .map_err(|err| Error::msg(err.to_string()))?;

        Ok(())
    }
}

#[pin_project]
pub struct PacketExtractor<const B: usize> {
    pub header: [u8; B],
    pub forwarded: usize,
    #[pin]
    pub socket: TcpStream,
}

impl<const B: usize> PacketExtractor<B> {
    pub async fn read(socket: TcpStream) -> Result<Self> {
        let mut extractor = Self {
            header: [0; B],
            forwarded: 0,
            socket,
        };

        extractor.socket.read_exact(&mut extractor.header).await?;

        Ok(extractor)
    }

    pub fn get_header(&mut self) -> &[u8; B] {
        &self.header
    }
}

impl<const B: usize> AsyncRead for PacketExtractor<B> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buff: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let extractor = self.project();

        if *extractor.forwarded < extractor.header.len() {
            let leftover = &extractor.header[*extractor.forwarded..];
            let num_forward_now = leftover.len().min(buff.remaining());
            let forward = &leftover[..num_forward_now];

            buff.put_slice(forward);
            *extractor.forwarded += num_forward_now;

            return Poll::Ready(Ok(()));
        }

        extractor.socket.poll_read(cx, buff)
    }
}

impl<const B: usize> AsyncWrite for PacketExtractor<B> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buff: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        let extractor = self.project();
        extractor.socket.poll_write(cx, buff)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), std::io::Error>> {
        let extractor = self.project();
        extractor.socket.poll_flush(cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        let extractor = self.project();
        extractor.socket.poll_shutdown(cx)
    }
}
