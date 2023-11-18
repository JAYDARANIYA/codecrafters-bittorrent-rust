mod bencode_decode;
mod cli_cmd;
mod models;

use std::io::Error;

use crate::{bencode_decode::decode_bencoded_values, models::info::MetaInfo};
use clap::Parser;
use cli_cmd::{Cli, Commands};
use models::{handshake::HandShake, tracker::TrackerRequest};

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.subcmd {
        Commands::Decode { encoded_value } => {
            let decoded_value = decode_bencoded_values(&encoded_value.as_bytes());

            for value in decoded_value.as_array().unwrap() {
                println!("{}", value);
            }
        }
        Commands::Info { path } => {
            let meta_info = MetaInfo::from_file(&path);

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
        }
        Commands::Peers { path } => {
            let meta_info = MetaInfo::from_file(&path);

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
        }
        Commands::Handshake { path, peer } => {
            let meta_info = MetaInfo::from_file(&path);

            let addr_port = &peer.split(":").collect::<Vec<&str>>();

            let mut handshake = HandShake::new(
                &meta_info.info_hash(),
                addr_port[0],
                addr_port[1].parse::<u16>().unwrap(),
                "00112233445566778899",
            );

            let peer_id = handshake.perform_handshake();

            println!("Peer ID: {}", hex::encode(peer_id));
        }
        Commands::DownloadPiece {
            out,
            path,
            piece_index,
        } => {
            let meta_info = MetaInfo::from_file(&path);

            let tracker_request = TrackerRequest::new(
                &meta_info.announce,
                &meta_info.info_hash(),
                "00112233445566778899".to_string(),
                6881,
                0,
                0,
                meta_info.info.length.to_string().as_str(),
            );

            let peers = tracker_request.get_peers();

            let peer = &peers[1];

            let mut handshake = HandShake::new(
                &meta_info.info_hash(),
                &peer.ip.as_str(),
                peer.port,
                "00112233445566778899",
            );

            let file_chunks = handshake.download_piece(piece_index as usize, &meta_info);

            std::fs::write(&out, file_chunks).expect("Unable to write file");

            println!("Piece {} downloaded to {}", piece_index, out);
        }

        Commands::Download { out, path } => {
            let meta_info = MetaInfo::from_file(&path);

            let tracker_request = TrackerRequest::new(
                &meta_info.announce,
                &meta_info.info_hash(),
                "00112233445566778899".to_string(),
                6881,
                0,
                0,
                meta_info.info.length.to_string().as_str(),
            );

            let peers = tracker_request.get_peers();

            let peer = &peers[1];

            let mut handshake = HandShake::new(
                &meta_info.info_hash(),
                &peer.ip.as_str(),
                peer.port,
                "00112233445566778899",
            );

            let file_chunks = handshake.download_all_pieces(meta_info);

            std::fs::write(&out, file_chunks).expect("Unable to write file");

            println!("Downloaded {} to {}", path, out);
        }
    }

    Ok(())
}
