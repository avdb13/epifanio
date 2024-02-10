use std::io::Write;

use bendy::encoding::{self, ToBencode};
use bytes::{Bytes, BytesMut, BufMut};
use rand::Rng;

use crate::{
    infohash::InfoHash,
    util::{PeerId, SocketAddr},
};

const PROTOCOL_ID: u64 = 0x41727101980;

pub trait Encode {
    fn encode(self, peer_id: PeerId) -> BytesMut;
}

pub trait Decode {
    fn encode(v: &[u8]) -> Self;
}

pub struct ConnId(u64);

impl Default for ConnId {
    fn default() -> Self {
        Self(rand::thread_rng().gen())
    }
}

impl From<ConnId> for u64 {
    fn from(value: ConnId) -> Self {
        value.0
    }
}

pub struct TxnId(u32);

impl Default for TxnId {
    fn default() -> Self {
        Self(rand::thread_rng().gen())
    }
}

impl From<TxnId> for u32 {
    fn from(value: TxnId) -> Self {
        value.0
    }
}

pub struct Status {
    downloaded: u64,
    left: u64,
    uploaded: u64,
}

pub enum Event {
    Completed = 1,
    Started,
    Stopped,
}

pub struct Request {
    txn_id: TxnId,
    parts: RequestParts,
}

pub enum RequestParts {
    Connect,
    Announce {
        conn_id: ConnId,
        info_hash: InfoHash,
        peer_id: PeerId,
        status: Status,
        event: Event,
        addr: SocketAddr,
        key: u32,
        num_want: Option<u32>,
    },
    Scrape {
        conn_id: ConnId,
        info_hash: InfoHash,
    },
}

impl Request {
    pub fn new(parts: RequestParts) -> Self {
        Self {
            parts,
            txn_id: TxnId::default(),
        }
    }
}

impl Encode for Request {
    fn encode(self) -> BytesMut {
        match self.parts {
            RequestParts::Connect => {
                let mut result = BytesMut::with_capacity(16);

                result.put_u64(PROTOCOL_ID);
                result.put_u32(0u32);
                result.put_u32(TxnId::default().into());

                result.to_owned()
            },
            RequestParts::Announce { conn_id, info_hash, peer_id, status, event, addr, key, num_want } => {
                let mut result = BytesMut::with_capacity(98);

                result.put_u64(conn_id.into());
                result.put_u32(1u32);
                result.put_u32(TxnId::default().into());
                result.put_slice(&info_hash.encode());
                result.put_slice(peer_id.into());
                result.put_u64(status.downloaded);
                result.put_u64(status.left);
                result.put_u64(status.uploaded);
                result.put_u32(event as u32);
                result.put_slice(addr.into());
                result.put_u32(key);
                result.put_u32(num_want.unwrap_or(0));
                result.put_u16(addr.into());

                result.to_owned()
            },
            RequestParts::Scrape { conn_id, info_hash } => todo!(),
        }
    }
}

pub struct ConnResponse {
    txn_id: TxnId,
    conn_id: ConnId,
}

pub struct AnnounceResponse {
    txn_id: TxnId,
    interval: u32,
    leechers: u32,
    seeders: u32,
    addrs: Vec<SocketAddr>,
}

#[derive(Default)]
pub struct Tracker {
    conn_id: ConnId,
}

impl Tracker {
    fn new() -> Self {
        Self::default()
    }

    fn connect() {}

    fn announce() {}

    fn scrape() {}
}
