use std::net::{TcpListener, TcpStream};

use sdl2::libc::NF_INET_FORWARD;

use crate::{animated_texture::AnimatedTexture, networking::{self, prepend_size, try_read_tcp}, packet::{ClientID, Packet}, player_packets::{PlayerAnimation, PlayerDisconnect, PlayerPacket, PlayerPosition, PlayerTextureData, PlayerWelcome}, texture_data::TextureData};

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
    
    
    if let Ok((mut socket, _)) = server.accept(){
        let mut test_packet = |packet: Packet| {
            // Send the packet
            networking::serialize_and_send(&mut client, packet.clone());
            // Read and deserialize the packet
            match try_read_tcp(&mut socket) {
                networking::NetworkResult::Ok(buf) => {
                    let deserialized_packet = networking::deserialize_to_packet(buf);
                    assert_eq!(deserialized_packet, Some(packet));
                },
                _ => panic!("Failed to read packet")
            }
        };
        
        test_packet(Packet::PlayerPacket(PlayerPacket::PlayerPositionPacket(PlayerPosition{x: 1, y: 2, player_id: 0})));
        test_packet(Packet::PlayerPacket(PlayerPacket::PlayerPositionPacket(PlayerPosition{x: 3, y: 4, player_id: 1})));
        test_packet(Packet::PlayerPacket(PlayerPacket::PlayerPositionPacket(PlayerPosition{x: 5, y: 6, player_id: 2})));
        test_packet(Packet::PlayerPacket(PlayerPacket::PlayerAnimationPacket(PlayerAnimation{id: 0, animation_data: AnimatedTexture::new(0.0)})));
        test_packet(Packet::PlayerPacket(PlayerPacket::PlayerDisconnectPacket(PlayerDisconnect{id: 0})));
        test_packet(Packet::ClientIDPacket(ClientID{id: 0}));
        test_packet(Packet::PlayerPacket(PlayerPacket::PlayerWelcomePacket(PlayerWelcome{player_id: 0, x : 12321, y : 102, texture_data : None})));
        test_packet(Packet::PlayerPacket(PlayerPacket::PlayerTextureDataPacket(PlayerTextureData{id : 123,texture_data : TextureData::new("resources/textures/water.png".to_string())})));
    }
}