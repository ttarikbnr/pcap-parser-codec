use futures::{Future, Stream};
use tokio::codec::Framed;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let fut = tokio::fs::File::open("example.pcap")
        .and_then(move |mut file| {
            let codec = pcap_parser::PcapCodec::new();
            let framed = Framed::new(file, codec);
            let (sink, stream) = framed.split();
            stream.for_each(move|packet| {
                println!("{}", packet.payload_len());
                Ok(())
            })
        })
        .map(|_| ())
        .map_err(|_| ());
    tokio::run(fut)
}
