mod bencode_decode;
mod models;
use std::env;

use models::tracker::TrackerRequest;

use crate::{bencode_decode::decode_bencoded_values, models::info::MetaInfo};

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let decoded_value = decode_bencoded_values(&args[2].as_bytes());

        for value in decoded_value.as_array().unwrap() {
            println!("{}", value);
        }
    } else if command == "info" {
        let meta_info = MetaInfo::from_file(&args[2]);

        println!("Tracker URL: {}", meta_info.announce);
        println!("Length: {}", meta_info.info.length);
        println!("Info Hash: {}", meta_info.info_hash_str());
        println!("Piece Length: {}", meta_info.info.piece_length);
        println!("Piece Hashes: ");

        for chunk in meta_info.info.pieces.chunks(20) {
            for x in chunk {
                print!("{:02x}", x);
            }
            println!("");
        }
    } else if command == "peers" {
        let meta_info = MetaInfo::from_file(&args[2]);

        let tracker_request = TrackerRequest::new(
            &meta_info.announce,
            &meta_info.info_hash(),
            "12345678901234567890".to_string(),
            6881,
            0,
            0,
            meta_info.info.length.to_string().as_str(),
        );

        let peers = tracker_request.get_peers();

        for peer in peers {
            println!("{}:{}", peer.ip, peer.port);
        }
    } else {
        println!("unknown command: {}", args[1])
    }
}
