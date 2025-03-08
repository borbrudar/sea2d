use crate::networking::{serialize_and_send, try_read_tcp, NetworkResult};
use crate::packet::{Packet, PacketInternal};
use crate::shared::*;
use std::io::{ErrorKind,Read,Write};
use std::net::TcpStream;
use std::sync::mpsc as mspc;
use std::thread;
use crate::game::Game;

pub fn client(address : &str ) {
    let mut client = TcpStream::connect(address).expect("Failed to connect");
    client.set_nonblocking(true).expect("Failed to initialize non-blocking client");
    println!("Running client on address {}",address);
    
    let (tx,rx) = mspc::channel::<Packet>(); // send from game thread to connection thread
    let (tx2 , rx2) = mspc::channel::<PacketInternal>(); // send to game thread from connection thread

    thread::spawn(move || loop{
        // read from server
        match try_read_tcp(&mut client){
            NetworkResult::Ok(buf) => {
                let packet_int  = bincode::deserialize(&buf);
                match packet_int{
                    Ok(packet_int) =>
                     tx2.send(packet_int).expect("Failed to send packet to game thread"),
                    Err(err) => println!("Failed to deserialize packet {:?}",err)
                }
            },
            NetworkResult::WouldBlock => (),
            NetworkResult::ConnectionLost => {
                println!("Connection lost client");
                break;
            }
        };
        
        // send to server
        match rx.try_recv(){
            // not ok, types not preserved
            Ok(packet) => serialize_and_send(&mut client, packet).unwrap(),
            Err(mspc::TryRecvError::Empty) => (),
            Err(mspc::TryRecvError::Disconnected) => {
                println!("Connection lost client");
                break;
            }
        }
    });
    
    // run game in main thread
    let mut game = Game::new(tx,rx2);
    game.run();
    println!("Bye bye!");
}