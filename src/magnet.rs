use thiserror::Error;

use crate::infohash::{InfoHash, InfoHashError};

const PREFIX: &str = "magnet:?";
const URN_V1: &str = "urn:btih:";
const URN_V2: &str = "urn:btmh:";

#[derive(Error, Debug)]
pub enum MagnetError {
    #[error("missing tracker")]
    MissingTracker,

    #[error("invalid tracker: {0}")]
    InvalidTracker(String),

    #[error("wrong prefix: {0}")]
    WrongPrefix(String),

    #[error("{0}")]
    InfoHash(#[from] InfoHashError),
}

#[derive(Debug)]
pub struct MagnetUri {
    pub hash: InfoHash,
    pub name: Option<String>,
    pub tracker_url: Option<String>,
    pub peer_addr: Option<String>,
}

impl TryFrom<&str> for MagnetUri {
    type Error = MagnetError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value
            .strip_prefix(PREFIX)
            .ok_or(MagnetError::WrongPrefix(value[..PREFIX.len()].to_owned()))?;

        let pairs: Vec<_> = value
            .split(|c| c == '&')
            .flat_map(|c| c.split_once('='))
            .collect();

        let find_pair = |s: &str| pairs.iter().find(|&&p| p.0 == s).map(|p| p.1.to_owned());

        let hash = match find_pair("xt").map(|s| s.to_owned()) {
            Some(xt) if xt.starts_with(URN_V1) && xt.len() - URN_V1.len() == 40 => {
                InfoHash::from_sha1(&xt[URN_V1.len()..])
            }
            Some(xt) if xt.starts_with(URN_V2) && xt.len() - URN_V2.len() == 64 => {
                InfoHash::from_sha256(&xt[URN_V2.len()..])
            }
            Some(xt) => {
                return Err(MagnetError::InvalidTracker(xt));
            }
            None => {
                return Err(MagnetError::MissingTracker);
            }
        };

        Ok(Self {
            hash,
            name: find_pair("dn"),
            tracker_url: find_pair("tr"),
            peer_addr: find_pair("x.pe"),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::infohash::InfoHash;

    use super::MagnetUri;

    #[test]
    fn decode() {
        let samples = [
            "magnet:?xt=urn:btih:807646161c8bad88781761dfc759eef870421098&dn=%5BLeopard-Raws%5D%\
             20%E3%81%AB%E3%82%83%E3%82%93%E3%81%93%E3%81%84%EF%BC%81%20%2307%20%28D-TBS%\
             201440x1080%29.rar&tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&tr=udp%3A%2F%\
             2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%\
             2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.\
             torrent.eu.org%3A451%2Fannounce",
            "magnet:?xt=urn:btmh:\
             1220caf1e1c30e81cb361b9ee167c4aa64228a7fa4fa9f6105232b28ad099f3a302e&\
             dn=bittorrent-v2-test",
            "magnet:?xt=urn:btih:631a31dd0a46257d5078c0dee4e66e26f73e42ac&xt=urn:btmh:\
             1220d8dd32ac93357c368556af3ac1d95c9d76bd0dff6fa9833ecdac3d53134efabb&\
             dn=bittorrent-v1-v2-hybrid-test",
            "magnet:?xt=urn:btih:6fce7b248f763460060de4698f49f1ebde390e4b&dn=One%20Piece%20%20%\
             20-%20%20%20Volumes%201%20-%2042&tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&\
             tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%\
             3A1337%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%\
             2Ftracker.torrent.eu.org%3A451%2Fannounce",
        ];

        for sample in samples {
            let ok = MagnetUri::try_from(sample).unwrap();
        }
    }

    fn encode() {
        let magnet = MagnetUri {
            hash: InfoHash::from_sha1("8a173fd3e32c0fa78b90fe42d305f202244e2739"),
            name: Some("サイコパス".to_owned()),
            tracker_url: Some("udp://tracker.torrent.eu.org:451/announce".to_owned()),
            peer_addr: Some("http://[1fff:0:a88:85a3::ac1f]:8001/index.html".to_owned()),
        };

        // assert_eq!(magnet.encode(), "magnet:?xt=urn:btih:8a173fd3e32c0fa78b90fe42d305f202244e2739&dn=%E3%82%B5%E3%82%A4%E3%82%B3%E3%83%91%E3%82%B9&tr=udp://tracker.torrent.eu.org:451/announce&x.pe=http://[1fff:0:a88:85a3::ac1f]:8001/index.html");
    }
}
