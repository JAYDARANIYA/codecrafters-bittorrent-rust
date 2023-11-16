mod bencode_decode;
mod models;
use std::{env, fs, path::Path};

use bencode_decode::decode_bencoded_file;

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
        let meta_info = MetaInfo::from_string(
            decode_bencoded_file(&bytes).as_str()
            );
        match meta_info {
            Some(m) => {
                println!("Tracker URL: {}",m.announce);
                println!("Length: {}",m.info.length);
            },
            None => {
                println!("Error parsing file");
            }
        }
    } else {
        println!("unknown command: {}", args[1])
    }
}
