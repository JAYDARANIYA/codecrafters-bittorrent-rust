use std::fmt::Write;

use reqwest::blocking::Client;

pub struct Peer {
    pub ip: String,
    pub port: u16,
}

pub struct TrackerRequest {
    url: String,
    info_hash: Vec<u8>,
    peer_id: String,
    port: u16,
    uploaded: usize,
    downloaded: usize,
    left: String,
}

impl TrackerRequest {
    pub fn new(
        url: &str,
        info_hash: &[u8],
        peer_id: String,
        port: u16,
        uploaded: usize,
        downloaded: usize,
        left: &str,
    ) -> TrackerRequest {
        TrackerRequest {
            url: url.to_string(),
            info_hash: info_hash.to_vec(),
            peer_id,
            port,
            uploaded,
            downloaded,
            left: left.to_string(),
        }
    }

    pub fn get_peers(&self) -> Vec<Peer> {
        let client = Client::new();
        let encoded_info_hash = self.info_hash.iter().fold(String::new(), |mut output, b| {
            let _ = write!(output, "%{b:02X}");
            output
        });

        let params = [
            ("peer_id", self.peer_id.as_str()),
            ("port", &self.port.to_string()),
            ("uploaded", &self.uploaded.to_string()),
            ("downloaded", &self.downloaded.to_string()),
            ("left", &self.left),
            ("compact", "1"),
        ];

        let params = serde_urlencoded::to_string(&params).expect("Failed to encode params");

        let url = format!("{}?{}&info_hash={}", self.url, params, encoded_info_hash);

        let response = client.get(&url).send().expect("Failed to send request");
        let body = response
            .bytes()
            .expect("Failed to get response body")
            .to_vec();

        let peers = serde_bencode::from_bytes::<serde_bencode::value::Value>(&body)
            .expect("Failed to decode response");

        let peers: Vec<Peer> = match peers {
            serde_bencode::value::Value::Dict(dict) => {
                let peers = dict.get("peers".as_bytes()).expect("No peers found");

                match peers {
                    serde_bencode::value::Value::Bytes(b) => {

                        b.chunks_exact(6)
                            .map(|chunk| Peer {
                                ip: format!("{}.{}.{}.{}", chunk[0], chunk[1], chunk[2], chunk[3]),
                                port: u16::from_be_bytes([chunk[4], chunk[5]]),
                            })
                            .collect()
                    }
                    _ => panic!("Expected dict"),
                }
            }
            _ => panic!("Expected dict"),
        };

        peers
    }
}
