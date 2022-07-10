use std::io;
use std::mem::MaybeUninit;
use std::net::{IpAddr, SocketAddr};

use pnet_packet::icmp::{self, IcmpTypes};
use pnet_packet::Packet;
use socket2::{Domain, Protocol, Socket, Type};
use tokio::io::unix::AsyncFd;
use tokio::select;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Pinger {
    seq: u16,
    addr: IpAddr,
    inner: AsyncFd<Socket>,
}

fn send(sock: &Socket, addr: &IpAddr, seq: u16) -> io::Result<()> {
    let mut buf = [0u8; icmp::echo_request::MutableEchoRequestPacket::minimum_packet_size()];
    let mut pkt = icmp::echo_request::MutableEchoRequestPacket::new(&mut buf).unwrap();
    pkt.set_icmp_type(IcmpTypes::EchoRequest);
    pkt.set_sequence_number(seq);

    let addr = SocketAddr::new(addr.clone(), 0);
    sock.send_to(pkt.packet(), &addr.into())?;

    Ok(())
}

fn recv(sock: &Socket) -> io::Result<icmp::echo_reply::EchoReplyPacket> {
    let mut buf = [MaybeUninit::new(0u8);
        icmp::echo_request::MutableEchoRequestPacket::minimum_packet_size()];
    let (_, _addr) = sock.recv_from(&mut buf)?;

    let buf = unsafe { &*(&buf[..] as *const [MaybeUninit<u8>] as *const [u8]) };

    let pkt = icmp::echo_reply::EchoReplyPacket::new(&buf).unwrap();
    Ok(pkt)
}

impl Pinger {
    pub fn open(addr: IpAddr) -> Result<Pinger> {
        let sock = match addr {
            IpAddr::V4(_) => Socket::new(
                Domain::IPV4,
                Type::DGRAM.nonblocking().cloexec(),
                Some(Protocol::ICMPV4),
            )?,
            IpAddr::V6(_) => Socket::new(
                Domain::IPV6,
                Type::DGRAM.nonblocking().cloexec(),
                Some(Protocol::ICMPV6),
            )?,
        };
        Ok(Pinger {
            seq: 0,
            addr,
            inner: AsyncFd::new(sock)?,
        })
    }

    pub async fn ping(&mut self) -> Result<()> {
        self.seq += 1;
        let seq = self.seq;

        let inner = &self.inner;
        let mut wrote = false;
        let mut read = false;

        while !wrote || !read {
            select! {
                w = inner.writable(), if !wrote => {
                    match w?.try_io(|fd| send(fd.get_ref(), &self.addr, seq)) {
                        Ok(result) => result?,
                        Err(_) => continue, // busy
                    };
                    wrote = true;
                }

                r = inner.readable(), if !read => {
                    let pkt = match r?.try_io(|fd| recv(fd.get_ref())) {
                        Ok(result) => result?,
                        Err(_) => continue, // busy
                    };
                    if pkt.get_sequence_number() == seq {
                        read = true;
                    }
                }
            }
        }

        Ok(()) // STUB
    }
}
