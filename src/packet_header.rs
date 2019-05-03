use nom::{be_u32, le_u32};


#[derive(Clone, Debug)]
pub struct PacketHeader {
    timestamp_ns: u64,
    payload_length: usize
}

impl PacketHeader {
    pub fn from_raw(raw_header: &PacketHeaderRaw, is_ns: bool) -> Self {
        let timestamp_ns = raw_header.get_timestamp_as_ns(is_ns);
        let payload_length = raw_header.get_packet_length();

        Self {
            timestamp_ns,
            payload_length
        }
    }
}

#[derive(Debug, Clone)]
pub struct PacketHeaderRaw {
    ts_sec      : u32,
    ts_usec     : u32,
    incl_len    : u32,
    orig_len    : u32,
}

impl PacketHeaderRaw {
    fn get_timestamp_as_ns(&self, is_ns: bool) -> u64 {
        let sec_as_ns = self.ts_sec  as u64 * 1_000_000_000;
        let sub_sec_as_ns = if is_ns {
            self.ts_usec as u64
        } else {
            self.ts_usec as u64 * 1000
        };

        sec_as_ns + sub_sec_as_ns
    }

    fn get_packet_length(&self) -> usize {
        self.orig_len as usize
    }
}



named!(pub parse_packet_header_be<PacketHeaderRaw>,
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

named!(pub parse_packet_header_le<PacketHeaderRaw>,
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