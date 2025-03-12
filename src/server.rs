use crate::networking::{deserialize_to_packet, serialize_and_send, try_read_tcp, NetworkResult};
use crate::shared::LOCAL;
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc as mspc, MutexGuard};
use std::thread;
use std::collections::{HashMap,HashSet};
use crate::packet::{ClientID, Packet};
use crate::player::Player;
use crate::player_packets::*;
use rand::Rng;

use std::sync::{Arc,Mutex};


fn new_client_id(set : &HashSet<u64> ) -> u64 {
    let mut rng = rand::rng();
    let mut random_u64: u64 = rng.random();
    while set.get(&random_u64).is_some() {
        random_u64 = rng.random();
    }
    random_u64
}



fn handle_player_send(packet : PlayerPacket, player_id : u64, players : &mut MutexGuard<'_,HashMap<u64,Player>>) -> Packet {
    match packet {
        PlayerPacket::PlayerPositionPacket(PlayerPosition{x,y, player_id}) => {
            players.get_mut(&player_id).unwrap().x = x;
            players.get_mut(&player_id).unwrap().y = y;
            return Packet::PlayerPacket(PlayerPacket::PlayerPositionPacket(PlayerPosition{player_id : player_id as u64, x, y}));
        },
        PlayerPacket::PlayerDisconnectPacket(PlayerDisconnect{id}) => {
            return Packet::PlayerPacket(PlayerPacket::PlayerDisconnectPacket(PlayerDisconnect{id}));
        },
        PlayerPacket::PlayerAnimationPacket(PlayerAnimation{id,animation_data}) => {
            players.get_mut(&player_id).unwrap().animation_data = Some(animation_data.clone());
            return Packet::PlayerPacket(PlayerPacket::PlayerAnimationPacket(PlayerAnimation{id,animation_data}));
        },
        PlayerPacket::PlayerWelcomePacket(PlayerWelcome{player_id,x,y}) => {
            return Packet::PlayerPacket(PlayerPacket::PlayerWelcomePacket(PlayerWelcome{player_id,x,y}));
        },
        _ => panic!("Wtf you doing bro")
    }
}


pub fn server(){
    // create a listener
    let listener = TcpListener::bind(LOCAL).expect("Failed to bind");
    listener.set_nonblocking(true).expect("Failed to initialize non-blocking");

    let mut clients : HashMap<u64,TcpStream> = HashMap::new(); // uuid to tcp stream
    let (tx,rx) = mspc::channel::<Packet>();   

    let ip_to_uuid = Arc::new(Mutex::new(HashMap::new()));
    let uuid_to_ip = Arc::new(Mutex::new(HashMap::new()));
    let mut used_uuid = HashSet::new();


    let players : Arc<Mutex<HashMap<u64,Player>>> = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // socket reading
        if let Ok((mut socket, addr)) = listener.accept() {
            println!("Client {} connected", addr);
            let tx= tx.clone();
            
            let new_id = new_client_id(&used_uuid);
            used_uuid.insert(new_id);
            let ip_to_uuid = Arc::clone(&ip_to_uuid);
            ip_to_uuid.lock().unwrap().insert(addr, new_id);
            uuid_to_ip.lock().unwrap().insert(new_id, addr);
            
            let players_loop = Arc::clone(&players);
            
            
            let mut players_lock = players_loop.lock().unwrap();
            tx.send(Packet::ClientIDPacket(ClientID{id : new_id})).expect("Failed to send client id packet");
            
            let uuid = *ip_to_uuid.lock().unwrap().get(&addr).unwrap();
            players_lock.insert(new_id, Player::new(uuid));

            
            clients.insert(uuid, socket.try_clone().expect("Failed to clone client"));
            let players_thr = Arc::clone(&players);
            
            // read from the socket, new thread for each client
            thread::spawn(move || loop {
                match try_read_tcp(&mut socket){
                    NetworkResult::Ok(buf) => {
                        match deserialize_to_packet(buf){
                            Some(Packet::PlayerPacket(packet)) => {
                                let sender_uuid = ip_to_uuid.lock().unwrap().get(&addr).unwrap().clone();
                                let mut players_lock = players_thr.lock().unwrap();
                                let packet = handle_player_send(packet, sender_uuid, &mut players_lock); 
                                tx.send(packet).expect("Failed to send player packet");
                            },
                            _ => println!("Unknown packet"),
                        }
                    },
                    NetworkResult::WouldBlock => (),
                    NetworkResult::ConnectionLost => {
                        println!("Closing connection with: {}", addr);
                        // send packet that signals client disconnect
                        let sender_uuid = ip_to_uuid.lock().unwrap().get(&addr).unwrap().clone();
                        tx.send(Packet::PlayerPacket(PlayerPacket::PlayerDisconnectPacket(PlayerDisconnect{id : sender_uuid as u64}))).expect("Failed to send disconnect packet");
                        break;
                    }
                }
                
            });
        }
        // socket writing
        if let Ok(msg) = rx.try_recv(){
            match msg {
                Packet::PlayerPacket(PlayerPacket::PlayerDisconnectPacket(PlayerDisconnect{id})) => {
                    clients.remove(&id);
                },
                _ => ()
            }

            for (_,mut client) in clients.iter_mut(){
                serialize_and_send(&mut client, msg.clone());
            }        
        }
    }
}

