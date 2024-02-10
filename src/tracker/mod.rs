use rand::Rng;

use crate::{
    infohash::InfoHash,
    util::{PeerId, SocketAddr},
};

pub trait TrackerLike {}

const PROTOCOL_ID: u64 = 0x41727101980;

pub struct ConnId(u64);

impl Default for ConnId {
    fn default() -> Self {
        Self(rand::thread_rng().gen())
    }
}

pub struct TxnId(u32);

impl Default for TxnId {
    fn default() -> Self {
        Self(rand::thread_rng().gen())
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
    Connect {
        protocol_id: u64,
    },
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

impl From<&RequestParts> for u8 {
    fn from(parts: &RequestParts) -> Self {
        match parts {
            RequestParts::Connect { .. } => 0,
            RequestParts::Announce { .. } => 1,
            RequestParts::Scrape { .. } => 2,
        }
    }
}

impl Request {
    fn new(parts: RequestParts) -> Self {
        Self {
            parts,
            txn_id: TxnId::default(),
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
