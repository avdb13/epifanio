pub struct PeerId([u8; 20]);

impl From<PeerId> for &[u8] {
    fn from(value: PeerId) -> Self {
        &value.0
    }
}

pub enum SocketAddr {
    V4(u32, u16),
    V6(u128, u16),
}

impl From<SocketAddr> for u16 {
    fn from(value: SocketAddr) -> Self {
        match value {
            SocketAddr::V4(_, port) => port,
            SocketAddr::V6(_, port) => port,
        }
    }
}

impl From<SocketAddr> for &[u8] {
    fn from(value: SocketAddr) -> Self {
        match value {
            SocketAddr::V4(addr, _) => &addr.to_be_bytes(),
            SocketAddr::V6(addr, _) => &addr.to_be_bytes(),
        }
    }
}
