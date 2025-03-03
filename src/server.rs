use crate::shared::{LOCAL, MSG_SIZE};
use std::io::{ErrorKind,Read,Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc as mspc, MutexGuard};
use std::thread;
use std::collections::{HashMap,HashSet};
use crate::packet::{ClientID, Packet, PacketInternal};
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

fn handle_player_receive(packet : Packet) -> Option<Vec<u8>>{
    match packet {
        Packet::PlayerPacket(in2) => {
            match in2 {   
                PlayerPacket::PlayerPositionPacket(inner) => {
                    let packet_int = PacketInternal::new(inner.clone()).unwrap();
                    return Some(bincode::serialize(&packet_int).unwrap());
                }
                PlayerPacket::PlayerWelcomePacket(inner) => {
                    let packet_int = PacketInternal::new(inner.clone()).unwrap();
                    return Some(bincode::serialize(&packet_int).unwrap());
                }
                PlayerPacket::PlayerTextureDataPacket(inner) => {
                    let packet_int = PacketInternal::new(inner.clone()).unwrap();
                    return Some(bincode::serialize(&packet_int).unwrap());
                }
                PlayerPacket::PlayerDisconnectPacket(inner) => {
                    let packet_int = PacketInternal::new(inner.clone()).unwrap();
                    return Some(bincode::serialize(&packet_int).unwrap());
                }
                PlayerPacket::PlayerAnimationPacket(inner) => {
                    let packet_int = PacketInternal::new(inner.clone()).unwrap();
                    return Some(bincode::serialize(&packet_int).unwrap());
                }
                _ => panic!("Wtf you doing bro")
            }
        }
        _ => panic!("Wtf are you sending")
    }
}

fn handle_player_send(packet : PlayerPacket, player_id : u64, players : &mut MutexGuard<'_,HashMap<u64,Player>>) -> Packet {
    match packet {
        PlayerPacket::PlayerMovementPacket(PlayerMovement{mov }) => {
            println!("Movement: {:?}", mov);     
            match mov {
                Movement::Up => {
                    players.get_mut(&player_id).unwrap().y += 15;
                }
                Movement::Down => {
                    players.get_mut(&player_id).unwrap().y -= 15;
                }
                Movement::Left => {
                    players.get_mut(&player_id).unwrap().x -= 15;
                }
                Movement::Right => {
                    players.get_mut(&player_id).unwrap().x += 15;
                }
            }
            return Packet::PlayerPacket(PlayerPacket::PlayerPositionPacket(PlayerPosition{player_id : player_id as u64,
                 x : players[&player_id].x as i32, y : players[&player_id].y}));
        },
        PlayerPacket::PlayerTextureDataPacket(PlayerTextureData{texture_data,id}) => {
            players.get_mut(&player_id).unwrap().texture_data = Some(texture_data);
            return Packet::PlayerPacket(PlayerPacket::PlayerTextureDataPacket(PlayerTextureData{texture_data : players[&player_id].texture_data.clone().unwrap(), id}));
        },
        PlayerPacket::PlayerDisconnectPacket(PlayerDisconnect{id}) => {
            return Packet::PlayerPacket(PlayerPacket::PlayerDisconnectPacket(PlayerDisconnect{id}));
        },
        PlayerPacket::PlayerAnimationPacket(PlayerAnimation{id,animation_data}) => {
            players.get_mut(&player_id).unwrap().animation_data = Some(animation_data.clone());
            return Packet::PlayerPacket(PlayerPacket::PlayerAnimationPacket(PlayerAnimation{id,animation_data}));
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
            tx.send(Packet::ClientIDPacket(ClientID{id : new_id})).expect("Failed to send player id packet");
            
            let uuid = *ip_to_uuid.lock().unwrap().get(&addr).unwrap();
            players_lock.insert(new_id, Player::new(uuid));

            // packet that tells everyone each other's initial position
            for u in players_lock.values(){
                tx.send(Packet::PlayerPacket(PlayerPacket::PlayerWelcomePacket(PlayerWelcome{player_id : u.id as u64, x : u.x, y : u.y, 
                    texture_data : u.texture_data.clone()}))).unwrap();
            }
            
            clients.insert(uuid, socket.try_clone().expect("Failed to clone client"));
            let players_thr = Arc::clone(&players);
            
            // read from the socket, new thread for each client
            thread::spawn(move || loop {
                println!("thread running");
                let mut buf = vec![0; MSG_SIZE];
                match socket.read_exact(&mut buf) {
                    Ok(_) => {
                        let raw = buf;
                        let packet_int = bincode::deserialize::<PacketInternal>(&raw);

                        match packet_int {
                            Ok(packet_int) => {
                                match packet_int.try_deserialize() {
                                    Some(Packet::PlayerPacket(packet)) => {
                                        let sender_uuid = ip_to_uuid.lock().unwrap().get(&addr).unwrap().clone();
                                        let mut players_lock = players_thr.lock().unwrap();
                                        let packet = handle_player_send(packet, sender_uuid, &mut players_lock);
                                        tx.send(packet).expect("Failed to send movement packet");
                                    }
                                    _ => println!("Unknown packet"),
                                }
                            },
                            Err(err) => 
                            println!("Failed to deserialize packet on server: {:?}", err),
                        }
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
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
            clients = clients.into_iter().filter_map(|mut client| {
            println!("Sending message to client {:?}", &msg.clone());
            
            let mut send : Option<Vec<u8>>;
            match msg.clone() {
                Packet::PlayerPacket(packet) => {
                    send = handle_player_receive(Packet::PlayerPacket(packet));
                }
                Packet::ClientIDPacket(packet) => {
                    let packet_int = PacketInternal::new(packet.clone()).unwrap();
                    send = Some(bincode::serialize(&packet_int).unwrap());
                }
            };
            
            match &mut send {
                Some (send) => {
                    if send.len() > MSG_SIZE {
                        panic!("Message length exceeded");
                    }
                    else{
                        send.append(&mut vec![0;MSG_SIZE - send.len()]);
                    }
                    client.1.write_all(&send).map(|_| client).ok()
                },
                None => None,
            }
            }).collect::<HashMap<u64,TcpStream>>();
        
        }
    }
}

