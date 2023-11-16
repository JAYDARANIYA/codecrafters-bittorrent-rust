use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaInfo {
    pub announce: String,
    #[serde(rename = "created by")]
    pub created_by: String,
    pub info: Info,
}

impl MetaInfo {
    pub fn from_string(s: &str) -> Option<MetaInfo> {
        match serde_json::from_str::<MetaInfo>(s) {
            Ok(m) => Some(m),
            Err(e) => {
                println!("Error parsing: {}",e);
                None
            },
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub length: i64,
    pub name: String,
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    pub pieces: String,
}
