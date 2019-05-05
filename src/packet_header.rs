use std::io::{Error, ErrorKind};
use nom::{be_u32, le_u32};
use bytes::{BytesMut, BufMut};

const PACKET_HEADER_LENGTH : usize = 16;

#[derive(Clone, Debug)]
pub struct PacketHeader {
    timestamp_ns: u64,
    payload_length: usize
}

impl PacketHeader {
    fn from_raw(raw_header: &PacketHeaderRaw, is_ns: bool) -> Self {
        let timestamp_ns = raw_header.timestamp_as_ns(is_ns);
        let payload_length = raw_header.payload_length();

        Self {
            timestamp_ns,
            payload_length
        }
    }

    pub fn payload_length(&self) -> usize {
        self.payload_length
    }

    pub fn timestamp_ns(&self) -> u64 {
        self.timestamp_ns
    }
}

#[derive(Debug, Clone)]
struct PacketHeaderRaw {
    ts_sec      : u32,
    ts_usec     : u32,
    incl_len    : u32,
    orig_len    : u32,
}

impl PacketHeaderRaw {
    fn timestamp_as_ns(&self, is_ns: bool) -> u64 {
        let sec_as_ns = self.ts_sec  as u64 * 1_000_000_000;
        let sub_sec_as_ns = if is_ns {
            self.ts_usec as u64
        } else {
            self.ts_usec as u64 * 1000
        };

        sec_as_ns + sub_sec_as_ns
    }

    fn payload_length(&self) -> usize {
        self.orig_len as usize
    }
}

pub fn parse_packet_header(src: &mut BytesMut, is_le: bool, is_ns: bool) -> Result<Option<PacketHeader>, Error> {
    if src.len() < PACKET_HEADER_LENGTH {
        return Ok(None)
    } 

    let packet_header_raw = {
        let res = if is_le {
                parse_packet_header_le(&src[0..PACKET_HEADER_LENGTH])
            } else {
                parse_packet_header_be(&src[0..PACKET_HEADER_LENGTH])
            };

        match res {
            Ok((_, header_raw)) => {
                header_raw
            }
            Err(err) => {
                let err_msg = format!("Couldn't parse packet header! {}", err);
                return Err(Error::new(ErrorKind::InvalidInput, err_msg));
            }
        }
    };

    src.split_to(PACKET_HEADER_LENGTH);
    Ok(Some(PacketHeader::from_raw(&packet_header_raw, is_ns)))
}



named!(parse_packet_header_be<PacketHeaderRaw>,
    do_parse!(
        ts_sec      : be_u32 >>
        ts_usec     : be_u32 >>
        incl_len    : be_u32 >>
        orig_len    : be_u32 >>
        (
            PacketHeaderRaw{
                ts_sec,
                ts_usec,
                incl_len,
                orig_len
            }
        )
    )
);

named!(parse_packet_header_le<PacketHeaderRaw>,
    do_parse!(
        ts_sec      : le_u32 >>
        ts_usec     : le_u32 >>
        incl_len    : le_u32 >>
        orig_len    : le_u32 >>
        (
            PacketHeaderRaw{
                ts_sec,
                ts_usec,
                incl_len,
                orig_len
            }
        )
    )
);