pub enum HashFn {
    SHA1 = 0x11,
    SHA2_256 = 0x12,
    SHA2_512 = 0x13,
    SHA3 = 0x14,
    BLAKE2b = 0x40,
    BLAKE2s = 0x41,
}

pub struct InfoHash {
    pub hash_fn: HashFn,
    pub value: Box<[u8]>,
}

impl InfoHash {
    pub fn from_sha1(v: &[u8]) -> Self {
        Self {
            hash_fn: HashFn::SHA1,
            value: Box::from(v),
        }
    }

    pub fn parse(v: &[u8]) -> Result<Self, ()> {
        if v.len() <= 4 {
            return Err(());
        }

        let metadata: [u8; 4] = v[..4].try_into().map_err(|_| ())?;

        let (_, hash) = v.split_at(4);
        let (code, length) = metadata.split_at(2);

        if hash.len() as u8 != u8::from_be_bytes(length.try_into().map_err(|_| ())?) {
            return Err(());
        }

        let hash_fn = match u8::from_be_bytes(code.try_into().map_err(|_| ())?) {
            0x11 => HashFn::SHA1,
            0x12 => HashFn::SHA2_256,
            0x13 => HashFn::SHA2_512,
            0x14 => HashFn::SHA3,
            0x40 => HashFn::BLAKE2b,
            0x41 => HashFn::BLAKE2s,
            _ => return Err(()),
        };

        Ok(Self {
            hash_fn,
            value: Box::from(&v[4..]),
        })
    }
}

pub struct PeerId(());

pub enum SocketAddr {
    V4([u8; 4], u32),
    V6([u16; 8], u32),
}
