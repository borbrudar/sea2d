use crate::shared::{LOCAL, MSG_SIZE};
use std::io::{ErrorKind,Read,Write};
use std::net::TcpListener;
use std::sync::{mpsc as mspc, MutexGuard};
use std::thread;
use std::collections::{HashMap,HashSet};
use crate::packet::{Packet, PacketInternal};
use crate::player::{Player, PlayerWelcome};
use rand::Rng;

use std::sync::{Arc,Mutex};
use crate::player::{PlayerMovement,Movement,PlayerPosition,PlayerPacket,PlayerID};


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
                PlayerPacket::PlayerIDPacket(inner) => {
                    let packet_int = PacketInternal::new(inner).unwrap();
                    return Some(bincode::serialize(&packet_int).unwrap());
                }
                PlayerPacket::PlayerPositionPacket(inner) => {
                    let packet_int = PacketInternal::new(inner.clone()).unwrap();
                    return Some(bincode::serialize(&packet_int).unwrap());
                }
                PlayerPacket::PlayerWelcomePacket(inner) => {
                    let packet_int = PacketInternal::new(inner.clone()).unwrap();
                    return Some(bincode::serialize(&packet_int).unwrap());
                }
                _ => panic!("Wtf you doing bro")
            }
        }
        _ => panic!("Wtf are you sending")
    }
}

fn handle_player_send(packet : PlayerPacket, player_id : usize, players : &mut MutexGuard<'_,Vec<Player>>) -> Packet {
    match packet {
        PlayerPacket::PlayerMovementPacket(PlayerMovement{mov }) => {
            println!("Movement: {:?}", mov);     
            match mov {
                Movement::Up => {
                    players[player_id].y += 15;
                }
                Movement::Down => {
                    players[player_id].y -= 15;
                }
                Movement::Left => {
                    players[player_id].x -= 15;
                }
                Movement::Right => {
                    players[player_id].x += 15;
                }
            }
            return Packet::PlayerPacket(PlayerPacket::PlayerPositionPacket(PlayerPosition{player_id : player_id as u64,
                 x : players[player_id].x, y : players[player_id].y}));
        },
        _ => panic!("Wtf you doing bro")
    }
}


pub fn server(){
    // create a listener
    let listener = TcpListener::bind(LOCAL).expect("Failed to bind");
    listener.set_nonblocking(true).expect("Failed to initialize non-blocking");

    let mut clients = vec![];
    let (tx,rx) = mspc::channel::<Packet>();   

    let ip_to_uuid = Arc::new(Mutex::new(HashMap::new()));
    let uuid_to_ip = Arc::new(Mutex::new(HashMap::new()));
    let uuid_to_player_id : Arc<Mutex<HashMap<u64, u64>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut used_uuid = HashSet::new();


    let players : Arc<Mutex<Vec<Player>>> = Arc::new(Mutex::new(Vec::new()));

    loop {
        // socket reading
        if let Ok((mut socket, addr)) = listener.accept() {
            println!("Client {} connected", addr);
            let tx= tx.clone();
            
            let id = new_client_id(&used_uuid);
            used_uuid.insert(id);
            let ip_to_uuid = Arc::clone(&ip_to_uuid);
            let uuid_to_player_id = Arc::clone(&uuid_to_player_id);
            ip_to_uuid.lock().unwrap().insert(addr, id);
            uuid_to_ip.lock().unwrap().insert(id, addr);
            
            let players = Arc::clone(&players);
            
            
            let mut players_lock = players.lock().unwrap();
            // add player id to map and send it to player so it knows its own id
            let player_id = players_lock.len() as u64;
            let ss = Packet::PlayerPacket(PlayerPacket::PlayerIDPacket(PlayerID{id : player_id}));
            tx.send(ss).expect("Failed to send player id packet");
            
            let uuid = *ip_to_uuid.lock().unwrap().get(&addr).unwrap();
            uuid_to_player_id.lock().unwrap().insert(uuid, player_id);
            players_lock.push(Player::new(player_id));

            // packet that tells everyone each other's initial position
            for i in 0..players_lock.len(){
                tx.send(Packet::PlayerPacket(PlayerPacket::PlayerWelcomePacket(PlayerWelcome{player_id : i as u64, x : players_lock[i].x, y : players_lock[i].y}))).unwrap();
            }
            
          
            clients.push(socket.try_clone().expect("Failed to clone client"));
            let players = Arc::clone(&players);
            
            // read from the socket, new thread for each client
            thread::spawn(move || loop {
                println!("thread running");
                let mut buf = vec![0; MSG_SIZE];
                match socket.read_exact(&mut buf) {
                    Ok(_) => {
                        let raw = buf.into_iter().collect::<Vec<_>>();
                        let packet_int = bincode::deserialize::<PacketInternal>(&raw);

                        match packet_int {
                            Ok(packet_int) => {
                                match packet_int.try_deserialize() {
                                    Some(Packet::PlayerPacket(packet)) => {
                                        let sender_uuid = ip_to_uuid.lock().unwrap().get(&addr).unwrap().clone();
                                        let player_id = uuid_to_player_id.lock().unwrap().get(&sender_uuid).unwrap().clone() as usize;
                                        let mut players_lock = players.lock().unwrap();
                                        let packet = handle_player_send(packet, player_id, &mut players_lock);
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
                        break;
                    }
                }
            });
        }
        // socket writing
        if let Ok(msg) = rx.try_recv(){
            clients = clients.into_iter().filter_map(|mut client| {
                println!("Sending message to client {:?}", &msg.clone());

                let mut send : Option<Vec<u8>>;
                match msg.clone() {
                    Packet::PlayerPacket(packet) => {
                        send = handle_player_receive(Packet::PlayerPacket(packet));
                    }
                    _ => panic!("Wtf are you sending")
                };
                
                match &mut send {
                    Some (send) => {
                        if send.len() > MSG_SIZE {
                            panic!("Message length exceeded");
                        }
                        else{
                            send.append(&mut vec![0;MSG_SIZE - send.len()]);
                        }
                        client.write_all(&send).map(|_| client).ok()
                    },
                    None => None,
                }
            }).collect::<Vec<_>>();
            
        }
    }
}

