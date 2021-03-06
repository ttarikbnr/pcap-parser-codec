use nom::{be_u32, be_u16, be_i32, le_u32, le_u16, le_i32};
use bytes::BytesMut;
use std::io::{Error, ErrorKind};

const NORMAL              : u32   = 0xa1b2c3d4;
const SWAPPED             : u32   = 0xd4c3b2a1;
const NORMAL_NS           : u32   = 0xa1b23c4d;
const SWAPPED_NS          : u32   = 0x4d3cb2a1;
const HEADER_LENGTH       : usize = 24;
const MAGIC_NUMBER_LENGTH : usize = 4;

pub fn parse_global_header(src: &mut BytesMut) -> Result<Option<HeaderRaw>, Error> {
    if src.len() < HEADER_LENGTH {
        return Ok(None)
    }

    let (_, magic_number) = be_u32(&src[0..MAGIC_NUMBER_LENGTH]).unwrap();

    if !is_pcap(magic_number) {
        return Err(Error::new(ErrorKind::InvalidData, "Wrong file format!"))
    }

    let header_raw = {
        let res = if is_little_endian(magic_number) {
                parse_header_le(&src[0..HEADER_LENGTH]) 
            } else {
                parse_header_be(&src[0..HEADER_LENGTH])
            };
        match res {
            Ok((_, header_raw)) => {
                header_raw
            }
            Err(err) => {
                let err_msg = format!("Couldn't parse global header! {}", err);
                return Err(Error::new(ErrorKind::InvalidInput, err_msg));
            }
        }
    };

    src.split_to(HEADER_LENGTH);

    return Ok(Some(header_raw))
}

#[derive(Debug, Clone)]
pub struct HeaderRaw {
    magic_number    : u32,
    version_major   : u16,
    version_minor   : u16,
    thiszone        : i32,
    sigfigs         : u32,
    snaplen         : u32,
    network         : u32
}

impl HeaderRaw {
    pub fn is_little_endian(&self) -> bool {
        is_little_endian(self.magic_number)
    }

    pub fn is_ns(&self) -> bool {
        self.magic_number == NORMAL_NS || self.magic_number == SWAPPED_NS
    }
}

fn is_pcap(magic_number: u32) -> bool {
    magic_number == NORMAL || magic_number == SWAPPED ||
    magic_number == NORMAL_NS || magic_number == SWAPPED_NS 
}

fn is_little_endian(magic_number: u32) -> bool {
    magic_number == SWAPPED || magic_number == SWAPPED_NS 
}


named!(parse_header_be<HeaderRaw>,
    do_parse!(
        magic_number    : be_u32 >>
        version_major   : be_u16 >>
        version_minor   : be_u16 >>
        thiszone        : be_i32 >>
        sigfigs         : be_u32 >>
        snaplen         : be_u32 >>
        network         : be_u32 >>
        (
            HeaderRaw {
                magic_number,
                version_major,
                version_minor,
                thiszone,
                sigfigs,
                snaplen,
                network
            }
        )
    ) 
);

named!(parse_header_le<HeaderRaw>,
    do_parse!(
        magic_number    : be_u32 >>
        version_major   : le_u16 >>
        version_minor   : le_u16 >>
        thiszone        : le_i32 >>
        sigfigs         : le_u32 >>
        snaplen         : le_u32 >>
        network         : le_u32 >>
        (
            HeaderRaw {
                magic_number,
                version_major,
                version_minor,
                thiszone,
                sigfigs,
                snaplen,
                network
            }
        )
    ) 
);

