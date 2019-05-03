use super::packet_header::PacketHeader;


#[derive(Clone, Debug)]
pub struct Packet {
    header: PacketHeader,
    payload: Vec<u8>
}

impl Packet {
    pub fn new(header: PacketHeader, payload: Vec<u8>) -> Self {
        Self {
            header,
            payload
        }
    }
}