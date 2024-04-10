use std::{
    io::Error,
    net::SocketAddr,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use anyhow::Result;
use atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use mhy_kcp::Kcp;
use prost::Message;
use tokio::{
    io::AsyncWrite,
    net::UdpSocket,
    sync::{Mutex, OnceCell},
};

use super::packet::CommandHandler;
use crate::{
    game::{GameContext, PlayerInfo},
    net::packet::Packet,
    util,
};

pub struct PlayerSession {
    pub token: u32,
    xor_pad: OnceCell<Box<[u8]>>,
    kcp: Mutex<Kcp<RemoteEndPoint>>,
    start_time_ms: u64,
    pub context: GameContext,
}

struct RemoteEndPoint {
    socket: Arc<UdpSocket>,
    addr: SocketAddr,
}

impl PlayerSession {
    pub fn new(socket: Arc<UdpSocket>, addr: SocketAddr, conv: u32, token: u32) -> Self {
        let player_info = PlayerInfo::new();

        Self {
            token,
            xor_pad: OnceCell::new(),
            kcp: Mutex::new(Kcp::new(
                conv,
                token,
                false,
                RemoteEndPoint { socket, addr },
            )),
            start_time_ms: util::cur_timestamp_ms(),
            context: GameContext::new(Arc::new(AtomicRefCell::new(player_info))),
        }
    }

    pub async fn consume(&mut self, buffer: &[u8]) -> Result<()> {
        {
            let mut kcp = self.kcp.lock().await;
            kcp.input(buffer)?;
            kcp.async_update(self.session_time() as u32).await?;
            kcp.async_flush().await?;
        }

        let mut packets = Vec::new();
        let mut buf = [0; 16384];
        while let Ok(length) = self.kcp.lock().await.recv(&mut buf) {
            if let Some(key) = self.xor_pad.get() {
                util::xor(&mut buf, key);
            }

            packets.push(Packet::from(&buf[..length]));
        }

        for packet in packets {
            Self::on_message(self, packet.cmd_type, packet.body).await?;
        }

        self.kcp
            .lock()
            .await
            .async_update(self.session_time() as u32)
            .await?;
        Ok(())
    }

    pub async fn send(&self, cmd_type: u16, body: impl Message) -> Result<()> {
        let mut buf = Vec::new();
        body.encode(&mut buf)?;

        let mut payload: Vec<u8> = Packet {
            cmd_type,
            head: Vec::new(),
            body: buf,
        }
        .into();

        if let Some(key) = self.xor_pad.get() {
            util::xor(&mut payload, key);
        }

        let mut kcp = self.kcp.lock().await;
        kcp.send(&payload)?;
        kcp.async_flush().await?;
        kcp.async_update(self.session_time() as u32).await?;

        Ok(())
    }

    pub fn player_info(&self) -> AtomicRef<PlayerInfo> {
        self.context.player.borrow()
    }

    pub fn player_info_mut(&self) -> AtomicRefMut<PlayerInfo> {
        self.context.player.borrow_mut()
    }

    pub fn player_uid(&self) -> u32 {
        self.player_info().uid
    }

    pub fn set_xorpad(&self, value: Box<[u8]>) {
        self.xor_pad.set(value).unwrap();
    }

    fn session_time(&self) -> u64 {
        util::cur_timestamp_ms() - self.start_time_ms
    }
}

// Auto-implemented
impl CommandHandler for PlayerSession {}

impl AsyncWrite for RemoteEndPoint {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        self.socket.poll_send_to(cx, buf, self.addr)
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }
}
