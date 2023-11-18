use std::{net::TcpStream, io::Read};

pub struct Peer {
    pub ip: String,
    pub port: u16,
}

pub struct PeerMessage {
    pub length: u32,
    pub message_type: PeerMessageType,
    pub payload: Vec<u8>,
}

impl PeerMessage {
    pub fn from_socket(stream: &mut TcpStream) -> Result<PeerMessage, String> {
        let mut length_buffer = [0; 4];
        stream
            .read_exact(&mut length_buffer)
            .expect("Failed to read from stream");

        let length = u32::from_be_bytes(length_buffer);

        let mut message_buffer = vec![0; length as usize];
        stream
            .read_exact(&mut message_buffer)
            .expect("Failed to read from stream");

        let message_type = PeerMessageType::from(&message_buffer[0]);

        let payload = message_buffer[1..].to_vec();

        Ok(PeerMessage {
            length,
            message_type,
            payload,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut message = vec![];
        message.extend(self.length.to_be_bytes().to_vec());
        message.push(u8::from(self.message_type.clone()));
        message.extend(self.payload.clone());

        message
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum PeerMessageType {
    Choke,
    Unchoke,
    Interested,
    NotInterested,
    Have,
    BitField,
    Request,
    Piece,
    Cancel,
    Port,
}

impl From<&u8> for PeerMessageType {
    fn from(byte: &u8) -> Self {
        match byte {
            0 => PeerMessageType::Choke,
            1 => PeerMessageType::Unchoke,
            2 => PeerMessageType::Interested,
            3 => PeerMessageType::NotInterested,
            4 => PeerMessageType::Have,
            5 => PeerMessageType::BitField,
            6 => PeerMessageType::Request,
            7 => PeerMessageType::Piece,
            8 => PeerMessageType::Cancel,
            9 => PeerMessageType::Port,
            _ => panic!("Unknown message type"),
        }
    }
}

impl From<PeerMessageType> for u8 {
    fn from(message: PeerMessageType) -> Self {
        match message {
            PeerMessageType::Choke => 0,
            PeerMessageType::Unchoke => 1,
            PeerMessageType::Interested => 2,
            PeerMessageType::NotInterested => 3,
            PeerMessageType::Have => 4,
            PeerMessageType::BitField => 5,
            PeerMessageType::Request => 6,
            PeerMessageType::Piece => 7,
            PeerMessageType::Cancel => 8,
            PeerMessageType::Port => 9,
        }
    }
}
