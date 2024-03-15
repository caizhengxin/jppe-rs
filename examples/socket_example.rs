#![feature(let_chains)]
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
// use std::io::BufReader;
use jppe_derive::{BorrowByteDecode, BorrowByteEncode};


#[derive(Debug, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode)]
pub struct Message<'a> {
    pub version: u8,
    #[jppe(byte_count=1)]
    pub body: MessageBody<'a>
}


#[derive(Debug, PartialEq, Eq, BorrowByteDecode, BorrowByteEncode)]
#[repr(u8)]
pub enum MessageBody<'a> {
    ReadReq {
        address: u8,
        length: u8,
    } = 1,
    ReadRsp {
        address: u8,
        length: u8,
        #[jppe(length="length")]
        data: &'a [u8],
    },
    WriteReq {
        address: u8,
        length: u8,
        #[jppe(length="length")]
        data: &'a [u8],    
    },
    WriteRsp {
        address: u8,
        length: u8,
    },
    Stop,
}


fn handle_connection(mut stream: TcpStream) {
    let mut input = [0; 1024];
    stream.read(&mut input).unwrap();
    let (_, value) = jppe::decode_borrow::<Message>(&input).unwrap();
    println!("receive req, {value:?}");
    assert_eq!(value.version, 1);

    match value.body {
        MessageBody::ReadReq { address, length } => {
            assert_eq!(address, 0x0002);
            assert_eq!(length, 3);

            // Send Read Response Command
            let value = Message {
                version: 1,
                body: MessageBody::ReadRsp { address: 0x0002, length: 3, data: b"\x00\x01\x02" },
            };
            stream.write_all(&jppe::encode_borrow(value)).unwrap();
        },
        MessageBody::WriteReq { address, length, data } => {
            assert_eq!(address, 0x0002);
            assert_eq!(length, 3);
            assert_eq!(data, b"\x00\01\x02");

            // Send Write Response Command
            let value = Message {
                version: 1,
                body: MessageBody::WriteRsp { address: 0x0002, length: 3 },
            };
            stream.write_all(&jppe::encode_borrow(value)).unwrap();
        },
        _ => {
            let value = Message {
                version: 1,
                body: MessageBody::Stop,
            };
            stream.write_all(&jppe::encode_borrow(value)).unwrap();
        },
    }
}


fn main() -> std::io::Result<()> {
    // Since the test case is directly used here, unwrap is used.

    std::thread::scope(|s| {
        s.spawn(|| {
            let listener = TcpListener::bind("127.0.0.1:6789").unwrap();

            // accept connections and process them serially
            for stream in listener.incoming() {
                println!("{stream:?}");
                handle_connection(stream.unwrap());
            }        
        });
        s.spawn(|| {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let mut stream = TcpStream::connect("127.0.0.1:6789").unwrap();

            // Send Read Command
            let value = Message {
                version: 1,
                body: MessageBody::ReadReq { address: 0x0002, length: 3 },
            };
            let encode_value = jppe::encode_borrow(value);
            stream.write_all(&encode_value).unwrap();
            let mut buf = [0; 10];
            stream.read(&mut buf).unwrap();
            let (_, value) = jppe::decode_borrow::<Message>(&buf).unwrap();
            println!("receive rsp, {value:?}");
            assert_eq!(value.version, 1);
            assert_eq!(value.body, MessageBody::ReadRsp { address: 0x0002, length: 3, data: b"\x00\x01\x02" });
        });
    });

    Ok(())
}