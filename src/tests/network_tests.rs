use std::net::{TcpListener, TcpStream};

use sdl2::libc::NF_INET_FORWARD;

use crate::{networking::{self, prepend_size, try_read_tcp}, packet::Packet, player_packets::{PlayerPacket, PlayerPosition}};

#[test]
fn prepend_size_test() {
    let mut v = vec![1,2,3,4,5];
    prepend_size(&mut v);
    assert_eq!(v,vec![5,0,1,2,3,4,5]);
}

#[test]
fn prepend_size_test_large() {
    let mut v = vec![1;65500];
    prepend_size(&mut v);
    assert_eq!(v.len(),65500+2);
    assert_eq!(v[0],220);
    assert_eq!(v[1],255);
}

#[test]
fn serialize_deserialize_test() {
    let server = TcpListener::bind("127.0.0.1:6000").unwrap();
    let mut client = TcpStream::connect("127.0.0.1:6000").unwrap();
    
    let packet = Packet::PlayerPacket(PlayerPacket::PlayerPositionPacket(PlayerPosition{player_id:25,x:-19,y:0}));
   
    if let Ok((mut socket, _)) = server.accept(){
        networking::serialize_and_send(&mut client, packet);
        match try_read_tcp(&mut socket) {
            networking::NetworkResult::Ok(buf) => {
                let packet = networking::deserialize_to_packet(buf);
                assert_eq!(packet,Some(Packet::PlayerPacket(PlayerPacket::PlayerPositionPacket(PlayerPosition{player_id:25,x:-19,y:0}))));
            },
            _ => panic!("Failed to read packet")
        }
    }
}