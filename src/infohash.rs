use thiserror::Error;

#[derive(Error, Debug)]
pub enum InfoHashError {
    #[error("multihash must be at least 4 characters long")]
    TooShort(usize),

    #[error("invalid hash function code: {0}")]
    UnknownCode(String),

    #[error("malformed length: {0}")]
    MalformedLength(String),

    #[error("malformed hash: found length {0} but expected {1}")]
    MalformedHash(usize, u8),
}

#[derive(Debug)]
pub enum HashFn {
    SHA1 = 0x11,
    SHA2_256 = 0x12,
}

#[derive(Debug)]
pub struct InfoHash {
    pub hash_fn: HashFn,
    pub value: Box<str>,
}

impl<'a> InfoHash {
    pub fn from_sha1(v: &str) -> Self {
        Self {
            hash_fn: HashFn::SHA1,
            value: Box::from(v),
        }
    }

    pub fn from_sha256(v: &str) -> Self {
        Self {
            hash_fn: HashFn::SHA2_256,
            value: Box::from(v),
        }
    }

    pub fn encode(self) -> [u8; 40] {
        let mut result = [0; 40];
        hex::encode_to_slice(self.value.as_bytes(), &mut result)
            .expect("couldn't encode hexadecimal form of hash");

        result
    }
}
