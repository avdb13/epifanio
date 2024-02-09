use crate::util::InfoHash;

const PREFIX: &str = "magnet:?";
const URN_V1: &str = "urn:btih:";
const URN_V2: &str = "urn:btmh:";

// v1: magnet:?xt=urn:btih:<info-hash>&dn=<name>&tr=<tracker-url>&x.pe=<peer-address>
// v2: magnet:?xt=urn:btmh:<tagged-info-hash>&dn=<name>&tr=<tracker-url>&x.pe=<peer-address>

pub struct MagnetUri {
    pub hash: InfoHash,
    pub name: Option<String>,
    pub tracker_url: Option<String>,
    pub peer_addr: Option<String>,
}

impl TryFrom<&str> for MagnetUri {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.strip_prefix(PREFIX).ok_or(())?;

        let pairs: Vec<_> = value
            .split(|c| c == '&')
            .flat_map(|c| c.split_once('='))
            .collect();

        let find_pair = |s: &str| pairs.iter().find(|&&p| p.0 == s).map(|p| p.1.to_owned());

        let hash = match find_pair("xt").map(|s| s.to_owned()) {
            Some(xt) if xt.starts_with(URN_V1) => InfoHash::from_sha1(xt.as_bytes()),
            Some(xt) if xt.starts_with(URN_V2) => InfoHash::parse(xt.as_bytes())?,
            Some(_) | None => return Err(()),
        };

        Ok(Self {
            hash,
            name: find_pair("dn"),
            tracker_url: find_pair("tr"),
            peer_addr: find_pair("x.pe"),
        })
    }
}

pub enum Message {
    Request,
    Data(Vec<u8>),
    Reject,
}

impl From<&Message> for u8 {
    fn from(parts: &Message) -> Self {
        match parts {
            Message::Request { .. } => 0,
            Message::Data { .. } => 1,
            Message::Reject { .. } => 2,
        }
    }
}
