use serde::Deserialize;
use serde::Serialize;
use serde_bytes::ByteBuf;

extern crate sha1;

use sha1::{Digest, Sha1};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaInfo {
    pub announce: String,
    pub info: Info,
}

impl MetaInfo {
    pub fn info_hash(&self) -> String {
        let serialized = serde_bencode::ser::to_bytes(&self.info).unwrap();
        let mut hasher = Sha1::new();
        hasher.update(&serialized);
        let result = hasher.finalize();

        format!("{:x}", result)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub length: i64,
    pub name: String,
    #[serde(rename = "piece length")]
    pub piece_length: u64,
    pub pieces: ByteBuf,
}
