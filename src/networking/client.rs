use crate::game::Game;
use crate::networking::packet::Packet;
use crate::networking::helpers::{NetworkResult, deserialize_to_packet, serialize_and_send, try_read_tcp};
use std::net::TcpStream;
use std::sync::mpsc as mspc;
use std::thread;

pub fn client(address: &str) {
    let mut client = TcpStream::connect(address).expect("Failed to connect");
    client
        .set_nonblocking(true)
        .expect("Failed to initialize non-blocking client");
    println!("Running client on address {}", address);

    let (tx, rx) = mspc::channel::<Packet>(); // send from game thread to connection thread
    let (tx2, rx2) = mspc::channel::<Packet>(); // send to game thread from connection thread

    thread::spawn(move || {
        loop {
            // read from server and send to game thread
            match try_read_tcp(&mut client) {
                NetworkResult::Ok(buf) => {
                    match deserialize_to_packet(buf) {
                        Some(packet) => tx2
                            .send(packet)
                            .expect("Failed to send packet to game thread"),
                        None => (),
                    };
                }
                NetworkResult::WouldBlock => (),
                NetworkResult::ConnectionLost => {
                    println!("Connection lost client");
                    break;
                }
            };

            // send to server
            match rx.try_recv() {
                Ok(packet) => serialize_and_send(&mut client, packet).unwrap(),
                Err(mspc::TryRecvError::Empty) => (),
                Err(mspc::TryRecvError::Disconnected) => {
                    println!("Connection lost client");
                    break;
                }
            }
        }
    });

    // run game in main thread
    let mut game = Game::new(tx, rx2);
    game.run();
    println!("Bye bye!");
}
