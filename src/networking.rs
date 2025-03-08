use std::net::TcpStream;
use std::io::{ErrorKind,Read};


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