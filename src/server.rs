use crate::networking::{deserialize_to_packet, serialize_and_send, try_read_tcp, NetworkResult};
use crate::shared::LOCAL;
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc as mspc, MutexGuard};
use std::thread;
use std::collections::{HashMap,HashSet};
use crate::packet::{ClientID, Packet, ServerInternal, ServerPacket};
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



fn handle_player_send(packet : PlayerPacket, player_id : u64, players : &mut HashMap<u64,Player>) -> Packet {
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
    let (client_sender,client_receiver) = mspc::channel::<Packet>();   // receives/send from/to clients directly
    let (server_sender,server_receiver) = mspc::channel::<ServerPacket>();   // intermediate step to handle input (internally so we dont need arc(mutex) on everything to pass between threads)

    let mut ip_to_uuid : HashMap<std::net::SocketAddr, u64> = HashMap::new();
    let mut uuid_to_ip : HashMap<u64,std::net::SocketAddr> = HashMap::new();
    let mut used_uuid = HashSet::new();


    let mut players : HashMap<u64,Player> = HashMap::new();

    loop {
        // socket reading
        if let Ok((mut socket, addr)) = listener.accept() {
            println!("Client {} connected", addr);
            let client_sender = client_sender.clone();
            let server_sender = server_sender.clone();

            // add a new uuid for the client 
            let uuid = new_client_id(&used_uuid);
            used_uuid.insert(uuid);
            ip_to_uuid.insert(addr,uuid);
            uuid_to_ip.insert(uuid,addr);
            
            // send the client id to the client
            client_sender.send(Packet::ClientIDPacket(ClientID{id : uuid})).expect("Failed to send client id packet");
            
            // add player to the player list
            server_sender.send(ServerPacket::AddPlayer(addr));            
            clients.insert(uuid, socket.try_clone().expect("Failed to clone client"));

            
            // read from the socket, new thread for each client
            thread::spawn(move || loop {
                match try_read_tcp(&mut socket){
                    NetworkResult::Ok(buf) => {
                        match deserialize_to_packet(buf){
                            // handling in the main thread
                            Some(packet) => server_sender.send(ServerPacket::ServerInternalPacket(ServerInternal{address:addr,packet})).expect("Failed to send player packet"),
                            _ => println!("Unknown packet"),
                        }
                    },
                    NetworkResult::WouldBlock => (),
                    NetworkResult::ConnectionLost => {
                        println!("Closing connection with: {}", addr);
                        // send packet that signals client disconnect
                        server_sender.send(ServerPacket::RemovePlayer(addr)).expect("Failed to send disconnect packet");
                        break;
                    }
                }
            });
        }
        // socket writing
        if let Ok(msg) = client_receiver.try_recv(){
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
        
        
        // handle incoming packets
        if let Ok(packet) = server_receiver.try_recv(){
            match packet{
                ServerPacket::ServerInternalPacket(packet) => {
                    let (addr, packet) = (packet.address, packet.packet);
                    match packet {
                        Packet::PlayerPacket(packet) => {
                            let sender_uuid = ip_to_uuid.get(&addr).unwrap().clone();
                            let packet = handle_player_send(packet, sender_uuid, &mut players); 
                            client_sender.send(packet).expect("Failed to send player packet");
                        }
                        _ => ()
                    }
                },
                ServerPacket::AddPlayer(addr) =>{
                    let uuid = *ip_to_uuid.get(&addr).unwrap();
                    players.insert(uuid, Player::new(uuid));
                },
                ServerPacket::RemovePlayer(addr) => {
                    let uuid = *ip_to_uuid.get(&addr).unwrap();
                    //players.remove(&uuid);
                    client_sender.send(Packet::PlayerPacket(PlayerPacket::PlayerDisconnectPacket(PlayerDisconnect{id : uuid as u64}))).expect("Failed to send disconnect packet");
                }
            }
        }

    }
}

