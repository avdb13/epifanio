use thiserror::Error;

#[derive(Error, Debug)]
pub enum InfoHashError {
    #[error("multihash must be at least 4 characters long")]
    TooShort(usize),

    #[error("unknown error")]
    TryInto,

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
    SHA2_512 = 0x13,
    SHA3 = 0x14,
    BLAKE2b = 0x40,
    BLAKE2s = 0x41,
}

#[derive(Debug)]
pub struct InfoHash {
    pub hash_fn: HashFn,
    pub value: Box<str>,
}

impl InfoHash {
    pub fn from_sha1(v: &str) -> Self {
        Self {
            hash_fn: HashFn::SHA1,
            value: Box::from(v),
        }
    }

    pub fn parse(v: &str) -> Result<Self, InfoHashError> {
        if v.len() <= 4 {
            return Err(InfoHashError::TooShort(v.len()));
        }

        let (metadata, hash) = v.split_at(4);
        let (code, length) = metadata.split_at(2);

        let expected_length = u8::from_str_radix(length, 16)
            .map_err(|_| InfoHashError::MalformedLength(length.to_owned()))?;
        if hash.len() as u8 != expected_length * 2 {
            return Err(InfoHashError::MalformedHash(hash.len(), expected_length));
        }

        let hash_fn = match code {
            "11" => HashFn::SHA1,
            "12" => HashFn::SHA2_256,
            "13" => HashFn::SHA2_512,
            "14" => HashFn::SHA3,
            "40" => HashFn::BLAKE2b,
            "41" => HashFn::BLAKE2s,
            _ => return Err(InfoHashError::UnknownCode(code.to_owned())),
        };

        Ok(Self {
            hash_fn,
            value: Box::from(hash),
        })
    }
}
