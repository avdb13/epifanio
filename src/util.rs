pub struct InfoHash(());

pub struct PeerId(());

pub enum SocketAddr {
    V4([u8; 4], u32),
    V6([u16; 8], u32),
}
