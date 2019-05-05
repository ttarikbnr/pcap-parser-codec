use super::packet_header::PacketHeader;
use std::io::{Error};
use bytes::{BytesMut, BufMut};


#[derive(Clone, Debug)]
pub struct Packet {
    ts_ns   : u64,
    payload : Vec<u8>
}

impl Packet {
    pub fn new(header: &PacketHeader, payload: Vec<u8>) -> Self {
        Self {
            ts_ns: header.timestamp_ns(),
            payload
        }
    }
}

pub fn parse_packet(src: &mut BytesMut, packet_header: &PacketHeader) -> Result<Option<Packet>, Error> {
    if src.len() < packet_header.payload_length(){
        return Ok(None)
    }

    let payload = src.split_to(packet_header.payload_length()).to_vec();

    Ok(Some(Packet::new(packet_header, payload)))
}