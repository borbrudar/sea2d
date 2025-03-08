use std::net::TcpStream;
use std::io::{ErrorKind,Read,Write};

use crate::packet::{Packet, PacketInternal};
use crate::shared::MAX_PACKET_SIZE;


pub enum NetworkResult{
    Ok(Vec<u8>),
    WouldBlock,
    ConnectionLost,
}

pub fn try_read_tcp(stream : &mut TcpStream) -> NetworkResult {
    let mut size = vec![0;2];
    match stream.read_exact(&mut size){
        Ok(_) => {
            let size = u16::from_le_bytes([size[0],size[1]]) as usize;
            let mut buf = vec![0;size];
            match stream.read_exact(&mut buf){
                Ok(_) => {
                    NetworkResult::Ok(buf)
                },
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => NetworkResult::WouldBlock,
                Err(_) => NetworkResult::ConnectionLost
            }
        },
        Err(ref err) if err.kind() == ErrorKind::WouldBlock => NetworkResult::WouldBlock,
        Err(_) => NetworkResult::ConnectionLost
    }
}

pub fn prepend_size(buf : &mut Vec<u8>) {
    let size = (buf.len() as u16).to_le_bytes();
    buf.insert(0,size[0]);
    buf.insert(1,size[1]);
    if buf.len() > MAX_PACKET_SIZE {
        panic!("Packet too large");
    }
}

pub fn serialize_and_send(stream : &mut TcpStream, packet : Packet) -> Option<()> {
    let packet_int = PacketInternal::new(packet.clone()).unwrap();
    let mut send = bincode::serialize(&packet_int).unwrap();
    prepend_size(&mut send);
    println!("message sent {:?}", packet);
    stream.write_all(&send).ok()
}