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
    is_le       : bool  // is little endian
}

impl PcapCodec {
    pub fn new() -> Self {
        Self {
            state       : State::ParsingGlobalHeader,
            is_ns       : true,
            is_le       : true
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
                    unimplemented!()
                }
                State::ParsingPacket => {
                    unimplemented!()
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