mod bencode_decode;
mod models;
use std::env;

use models::{handshake::HandShake, tracker::TrackerRequest};

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
    } else if command == "handshake" {
        let meta_info = MetaInfo::from_file(&args[2]);

        let addr_port = &args[3].split(":").collect::<Vec<&str>>();

        let handshake = HandShake::new(
            &meta_info.info_hash(),
            addr_port[0],
            addr_port[1].parse::<u16>().unwrap(),
            "00112233445566778899",
        );

        let peer_id = handshake.perform_handshake();

        println!("Peer ID: {}", hex::encode(peer_id));
    } else {
        println!("unknown command: {}", args[1])
    }
}
