mod bencode_decode;
use std::env;

use crate::bencode_decode::decode_bencoded_values;

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let decoded_value = decode_bencoded_values(&args[2].as_bytes());

        for value in decoded_value.as_array().unwrap() {
            println!("{}", value);
        }
    } else {
        println!("unknown command: {}", args[1])
    }
}
