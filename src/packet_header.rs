use nom::{be_u32, le_u32};

pub struct PacketHeader {
    timestamp: u64,
    packet_length: u32
}

#[derive(Debug, Clone)]
struct PacketHeaderRaw {
    ts_sec      : u32,
    ts_usec     : u32,
    incl_len    : u32,
    orig_len    : u32,
}


named!(parse_packet_header<PacketHeaderRaw>,
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


named!(parse_packet_header_swapped<PacketHeaderRaw>,
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