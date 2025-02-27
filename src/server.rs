use crate::shared::{LOCAL, MSG_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::io::{ErrorKind,Read,Write};
use std::net::TcpListener;
use std::sync::mpsc as mspc;
use std::thread;
use std::collections::{HashMap,HashSet};
use crate::packet::{Packet, PacketInternal, PlayerID,PlayerPosition};
use rand::Rng;
use crate::packet::{PlayerMovement,Movement};
use std::sync::{Arc,Mutex};


fn new_client_id(set : &HashSet<u64> ) -> u64 {
    let mut rng = rand::rng();
    let mut random_u64: u64 = rng.random();
    while set.get(&random_u64).is_some() {
        random_u64 = rng.random();
    }
    random_u64
}

pub fn server(){
    // create a listener
    let listener = TcpListener::bind(LOCAL).expect("Failed to bind");
    listener.set_nonblocking(true).expect("Failed to initialize non-blocking");

    let mut clients = vec![];
    let (tx,rx) = mspc::channel::<Packet>();   

    let ip_to_uuid = HashMap::new();
    let mut uuid_to_ip = HashMap::new();
    let mut used_uuid = HashSet::new();

    let player_positions : Arc<Mutex<Vec<(i32,i32)>>> = Arc::new(Mutex::new(vec![]));

    loop {
        // socket reading
        if let Ok((mut socket, addr)) = listener.accept() {
            println!("Client {} connected", addr);
            let tx= tx.clone();
            let mut ip_to_uuid = ip_to_uuid.clone();
            
            let id = new_client_id(&used_uuid);
            used_uuid.insert(id);
            //ip_to_uuid.insert(addr, id);
            //uuid_to_ip.insert(id, addr);
            ip_to_uuid.insert(addr, clients.len() as u64);
            uuid_to_ip.insert(clients.len() as u64, addr);
            
            let ss = Packet::PlayerIDPacket(PlayerID{id : clients.len() as u64});
            tx.send(ss).expect("Failed to send player id packet");
            let player_positions = Arc::clone(&player_positions);

            {
                let mut positions = player_positions.lock().unwrap();
                positions.push(((SCREEN_WIDTH as i32)/2,(SCREEN_HEIGHT as i32)/2));
            }
          
          
            clients.push(socket.try_clone().expect("Failed to clone client"));
            
            let player_positions = Arc::clone(&player_positions);
            // read from the socket, new thread for each client
            thread::spawn(move || loop {
                println!("thread running");
                let mut buf = vec![0; MSG_SIZE];
                match socket.read_exact(&mut buf) {
                    Ok(_) => {
                        let raw = buf.into_iter().collect::<Vec<_>>();
                        let packet_int = bincode::deserialize::<PacketInternal>(&raw);
                        let ref_ip_to_uuid = &ip_to_uuid;

                        match packet_int {
                            Ok(packet_int) => {
                                match packet_int.try_deserialize() {
                                    Some(Packet::PlayerMovementPacket(PlayerMovement {mov})) => {
                                        println!("Movement: {:?}", mov);
                                        let sender_id = ref_ip_to_uuid.get(&addr).unwrap().clone();
                                        let mut positions = player_positions.lock().unwrap();
                                        
                                        match mov {
                                            Movement::Up => {
                                                positions[sender_id as usize].1 += 15;
                                            }
                                            Movement::Down => {
                                                positions[sender_id as usize].1 -= 15;
                                            }
                                            Movement::Left => {
                                                positions[sender_id as usize].0 -= 15;
                                            }
                                            Movement::Right => {
                                                positions[sender_id as usize].0 += 15;
                                            }
                                        }
                                        let packet = Packet::PlayerPositionPacket(PlayerPosition{player_id : sender_id,
                                             x : positions[sender_id as usize].0, y : positions[sender_id as usize].1});
                                        println!("packet received and transferred in server {:?}", packet);
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

                //thread::sleep(::std::time::Duration::from_millis(100));
            });
        }
        // socket writing
        if let Ok(msg) = rx.try_recv(){
            clients = clients.into_iter().filter_map(|mut client| {
                println!("Sending message to client {:?}", &msg.clone());

                let mut send : Option<Vec<u8>> = None;
                match msg.clone() {
                    Packet::PlayerIDPacket(inner) => {
                        let packet_int = PacketInternal::new(inner).unwrap();
                        send = Some(bincode::serialize(&packet_int).unwrap());
                    }
                    Packet::PlayerPositionPacket(inner) => {
                        let packet_int = PacketInternal::new(inner.clone()).unwrap();
                        send = Some(bincode::serialize(&packet_int).unwrap());
                        println!("Sending player position packet {:?}", &inner);
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

