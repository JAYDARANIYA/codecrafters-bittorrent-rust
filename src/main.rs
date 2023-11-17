mod bencode_decode;
mod models;
use std::{env, fs, path::Path};

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
        let bytes = fs::read(Path::new(&args[2])).expect("No file found");
        // let meta_info = MetaInfo::from_string(decode_bencoded_file(&bytes).as_str());

        let meta_info = serde_bencode::from_bytes::<MetaInfo>(&bytes).expect("Error decoding file");

        println!("Tracker URL: {}", meta_info.announce);
        println!("Length: {}", meta_info.info.length);
        println!("Info Hash: {}",meta_info.info_hash());
        println!("Piece Length: {}", meta_info.info.piece_length);
        println!("Piece Hashes: ");

        for chunk in meta_info.info.pieces.chunks(20) {
            for x in chunk {
                print!("{:02x}",x);
            }
            println!("");
        }

    } else {
        println!("unknown command: {}", args[1])
    }
}
