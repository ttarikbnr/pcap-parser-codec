use nom::{be_u32, be_u16, be_i32, le_u32, le_u16, le_i32};

const NORMAL     : u32 = 0xa1b2c3d4;
const SWAPPED    : u32 = 0xd4c3b2a1; 
const NORMAL_NS  : u32 = 0xa1b23c4d;
const SWAPPED_NS : u32 = 0x4d3cb2a1;

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
    pub fn is_swapped(&self) -> bool {
        self.magic_number == SWAPPED || self.magic_number == SWAPPED_NS 
    }

    pub fn is_ns(&self) -> bool {
        self.magic_number == NORMAL_NS || self.magic_number == SWAPPED_NS
    }
}

named!(parse_header<HeaderRaw>,
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

named!(parse_header_swapped<HeaderRaw>,
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

