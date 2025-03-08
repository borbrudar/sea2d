use crate::networking::{try_read_tcp, NetworkResult};
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
            Ok(msg) => {
                let packet_int = PacketInternal::new(msg.clone()).unwrap();
                let mut send = bincode::serialize(&packet_int).unwrap();
                let size = (send.len() as u16).to_le_bytes();
                send.insert(0, size[1]);
                send.insert(0, size[0]);

                if send.len() > MAX_PACKET_SIZE {
                    panic!("Max packet size exceeded");
                }

                client.write_all(&send).expect("Writing to socket failed");
                println!("message sent {:?}", msg);
            },
            Err(mspc::TryRecvError::Empty) => (),
            Err(mspc::TryRecvError::Disconnected) => {
                println!("Connection lost client");
                break;
            }
        }
    });
    
    let mut game = Game::new(tx,rx2);
    game.run();
    println!("Bye bye!");
}