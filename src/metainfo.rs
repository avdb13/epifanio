use crate::infohash::InfoHash;

pub struct MetaInfo {
    info: Info,

    announce: String,
    announce_list: Vec<String>,

    creation_date: usize,
    comment: String,
    created_by: String,
    encoding: String,
}

pub struct Info {
    piece_length: usize,
    pieces: Vec<InfoHash>,
    private: bool,
    rest: InfoPart,
}

pub enum InfoPart {
    Single {
        name: String,
        length: usize,
        md5sum: Option<[u8; 32]>,
    },
    Multi {
        name: String,
        files: Vec<FileInfo>,
    },
}

pub struct FileInfo {
    length: usize,
    md5sum: Option<[u8; 32]>,
    path: String,
}
