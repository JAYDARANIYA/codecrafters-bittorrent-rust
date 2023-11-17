use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub struct HandShake {
    pub info_hash: Vec<u8>,
    pub addr: String,
    pub port: u16,
    pub peer_id: String,
}

impl HandShake {
    pub fn new(info_hash: &[u8], addr: &str, port: u16, peer_id: &str) -> HandShake {
        HandShake {
            info_hash: info_hash.to_vec(),
            addr: addr.to_string(),
            port,
            peer_id: peer_id.to_string(),
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
    pub fn perform_handshake(&self) -> Vec<u8> {
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

        // return peer id
        response[response.len() - 20..].to_vec()
    }
}
