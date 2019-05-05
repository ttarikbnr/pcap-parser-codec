#[macro_use]
extern crate nom;

mod header;
mod packet_header;
mod packet;

use bytes::{BytesMut, BufMut};
use tokio::codec::{Encoder, Decoder};
use packet::Packet;



enum State{
    ParsingGlobalHeader,
    ParsingPacketHeader,
    ParsingPacket
}

pub struct PcapCodec {
    state       : State,
    is_ns       : bool, 
    is_le       : bool,  // is little endian
    current_packet_header: Option<packet_header::PacketHeader>
}

impl PcapCodec {
    pub fn new() -> Self {
        Self {
            state       : State::ParsingGlobalHeader,
            is_ns       : true,
            is_le       : true,
            current_packet_header: None
        }
    }
}

impl Decoder for PcapCodec {
    type Item = Packet;
    type Error = std::io::Error;

    fn decode(&mut self, 
              src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        loop {
            let new_state = match self.state{
                State::ParsingGlobalHeader => {
                    if let Some(header) = header::parse_global_header(src)?{
                        self.is_ns = header.is_ns();
                        self.is_le = header.is_little_endian();
                        State::ParsingPacketHeader
                    } else {
                        return Ok(None)
                    }
                }
                State::ParsingPacketHeader => {
                    if let Some(packet_header) = packet_header::parse_packet_header(src, self.is_le, self.is_ns)? {
                        self.current_packet_header = Some(packet_header);
                        State::ParsingPacket
                    } else {
                        return Ok(None)
                    }
                }
                State::ParsingPacket => {
                    if let Some(header) = self.current_packet_header.as_ref() {
                        if let Some(packet) = packet::parse_packet(src, header)?{
                            self.state = State::ParsingPacketHeader;
                            return Ok(Some(packet))
                        } else {
                            return Ok(None)
                        }
                    } else {
                        unreachable!()
                    }
                }
            };
            self.state = new_state;
        }
    }
}


impl Encoder for PcapCodec {
    type Item = Packet;
    type Error = std::io::Error;

    fn encode(&mut self,
              _item: Self::Item,
              _dst: &mut BytesMut) -> Result<(), Self::Error> {
        return Ok(())
    }
}