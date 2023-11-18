use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::models::peers::{PeerMessage, PeerMessageType};

use super::info::MetaInfo;

const KB_16: usize = 16 * 1024;

pub struct HandShake {
    pub info_hash: Vec<u8>,
    pub addr: String,
    pub port: u16,
    pub peer_id: String,
    pub socket: Option<TcpStream>,
}

impl HandShake {
    pub fn new(info_hash: &[u8], addr: &str, port: u16, peer_id: &str) -> HandShake {
        HandShake {
            info_hash: info_hash.to_vec(),
            addr: addr.to_string(),
            port,
            peer_id: peer_id.to_string(),
            socket: None,
        }
    }

    pub fn get_handshake(&self) -> Vec<u8> {
        let mut handshake = vec![19];
        handshake.extend(b"BitTorrent protocol");
        handshake.extend(vec![0; 8]);
        handshake.extend(self.info_hash.clone());
        handshake.extend(self.peer_id.clone().into_bytes());

        handshake
    }

    // perform handshake and return peer id
    pub fn perform_handshake(&mut self) -> Vec<u8> {
        let mut stream = TcpStream::connect(format!("{}:{}", self.addr, self.port))
            .expect("Failed to connect to peer");

        let handshake = self.get_handshake();

        stream
            .write_all(&handshake)
            .expect("Failed to write to stream");

        // read handshake response
        let mut response = vec![0; 68];
        stream
            .read_exact(&mut response)
            .expect("Failed to read from stream");

        self.socket = Some(stream);

        // return peer id
        response[response.len() - 20..].to_vec()
    }

    pub fn download_piece(&mut self, piece_index: usize, meta_info: MetaInfo) -> Vec<u8> {
        let mut stream = match &mut self.socket {
            Some(stream) => stream,
            None => panic!("No socket"),
        };

        // wait for bitfield message
        let message = PeerMessage::from_socket(&mut stream).expect("Failed to read from stream");

        if message.message_type != PeerMessageType::BitField {
            panic!("Expected bitfield message");
        }

        // send interested message
        let interested_message = PeerMessage {
            length: 1,
            message_type: PeerMessageType::Interested,
            payload: vec![],
        };

        stream
            .write_all(&interested_message.to_bytes())
            .expect("Failed to write to stream");

        // wait for unchoke message
        let message = PeerMessage::from_socket(&mut stream).expect("Failed to read from stream");

        if message.message_type != PeerMessageType::Unchoke {
            panic!("Expected unchoke message");
        }

        // send request message
        let mut offset = 0;
        let file_length = meta_info.info.length;
        let piece_length = meta_info.info.piece_length;
        let length = (file_length as usize - (piece_index * piece_length as usize))
            .min(piece_length as usize) as usize;
        let mut chunks: Vec<u8> = Vec::new();

        while offset < length {
            let block_length = KB_16.min(length - offset);
            let mut payload: Vec<u8> = Vec::new();
            payload.extend((piece_index as u32).to_be_bytes());
            payload.extend((offset as u32).to_be_bytes());
            payload.extend((block_length as u32).to_be_bytes());

            let request_message = PeerMessage {
                length: (payload.len() + 1) as u32,
                message_type: PeerMessageType::Request,
                payload,
            };

            stream
                .write_all(&request_message.to_bytes())
                .expect("Failed to write to stream");

            // wait for piece message

            let message =
                PeerMessage::from_socket(&mut stream).expect("Failed to read from stream");

            if message.message_type != PeerMessageType::Piece {
                panic!("Expected piece message");
            }

            chunks.extend(&message.payload[8..]);
            offset += block_length;

            if offset == length {
                break;
            }
        }

        chunks
    }
}
