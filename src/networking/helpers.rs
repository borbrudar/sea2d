use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;

use crate::networking::packet::{ClientID, Packet, PacketInternal};
use crate::networking::player_packets::{
    PlayerAnimation, PlayerDisconnect, PlayerLevel, PlayerPacket, PlayerPosition, PlayerWelcome,
};
use crate::networking::shared::MAX_PACKET_SIZE;

pub enum NetworkResult {
    Ok(Vec<u8>),
    WouldBlock,
    ConnectionLost,
}

pub fn try_read_tcp(stream: &mut TcpStream) -> NetworkResult {
    let mut size = vec![0; 2];
    match stream.read_exact(&mut size) {
        Ok(_) => {
            let size = u16::from_le_bytes([size[0], size[1]]) as usize;
            let mut buf = vec![0; size];
            match stream.read_exact(&mut buf) {
                Ok(_) => NetworkResult::Ok(buf),
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => NetworkResult::WouldBlock,
                Err(_) => NetworkResult::ConnectionLost,
            }
        }
        Err(ref err) if err.kind() == ErrorKind::WouldBlock => NetworkResult::WouldBlock,
        Err(_) => NetworkResult::ConnectionLost,
    }
}

pub fn prepend_size(buf: &mut Vec<u8>) {
    let size = (buf.len() as u16).to_le_bytes();
    buf.insert(0, size[0]);
    buf.insert(1, size[1]);
    if buf.len() > MAX_PACKET_SIZE {
        panic!("Packet too large");
    }
}

pub fn serialize_and_send(stream: &mut TcpStream, packet: Packet) -> Option<()> {
    //println!("serializing packet {:?}", packet);
    let packet_int = match packet.clone() {
        Packet::ClientIDPacket(inner) => PacketInternal::new(inner).unwrap(),
        Packet::PlayerPacket(PlayerPacket::PlayerAnimationPacket(inner)) => {
            PacketInternal::new(inner).unwrap()
        }
        Packet::PlayerPacket(PlayerPacket::PlayerDisconnectPacket(inner)) => {
            PacketInternal::new(inner).unwrap()
        }
        Packet::PlayerPacket(PlayerPacket::PlayerPositionPacket(inner)) => {
            PacketInternal::new(inner).unwrap()
        }
        Packet::PlayerPacket(PlayerPacket::PlayerWelcomePacket(inner)) => {
            PacketInternal::new(inner).unwrap()
        }
        Packet::PlayerPacket(PlayerPacket::PlayerLevelPacket(inner)) => {
            PacketInternal::new(inner).unwrap()
        }
    };
    //println!("internal packet {:?}", packet_int);
    let mut send = bincode::serialize(&packet_int).unwrap();
    prepend_size(&mut send);
    //println!("message sent {:?}", packet);
    //println!("data sent {:?}", send);
    stream.write_all(&send).ok()
}

pub fn deserialize_to_packet(buf: Vec<u8>) -> Option<Packet> {
    let packet_int = bincode::deserialize::<PacketInternal>(&buf);
    //println!("Received packet: {:?}", packet_int);

    match packet_int {
        Ok(packet_int) => {
            if let Some(packet) = packet_int.try_deserialize::<ClientID>() { return Some(Packet::ClientIDPacket(packet)) };

            if let Some(packet) = packet_int.try_deserialize::<PlayerAnimation>() {
                return Some(Packet::PlayerPacket(PlayerPacket::PlayerAnimationPacket(
                    packet,
                )));
            };

            if let Some(packet) = packet_int.try_deserialize::<PlayerPosition>() {
                return Some(Packet::PlayerPacket(PlayerPacket::PlayerPositionPacket(
                    packet,
                )));
            };

            if let Some(packet) = packet_int.try_deserialize::<PlayerWelcome>() {
                return Some(Packet::PlayerPacket(PlayerPacket::PlayerWelcomePacket(
                    packet,
                )));
            };

            if let Some(packet) = packet_int.try_deserialize::<PlayerDisconnect>() {
                return Some(Packet::PlayerPacket(PlayerPacket::PlayerDisconnectPacket(
                    packet,
                )));
            };
            if let Some(packet) = packet_int.try_deserialize::<PlayerLevel>() {
                return Some(Packet::PlayerPacket(PlayerPacket::PlayerLevelPacket(
                    packet,
                )));
            }
            None
        }
        Err(err) => panic!("Failed to deserialize packet: {:?}", err),
    }
}
