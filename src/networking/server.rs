use crate::networking::packet::{ClientID, Packet, ServerInternal, ServerPacket};
use crate::networking::{NetworkResult, deserialize_to_packet, serialize_and_send, try_read_tcp};
use crate::networking::{player_packets::*, shared::LOCAL};
use crate::player::Player;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc as mspc;
use std::thread;

fn new_client_id(set: &HashSet<u64>) -> u64 {
    let mut rng = rand::rng();
    let mut random_u64: u64 = rng.random();
    while set.get(&random_u64).is_some() {
        random_u64 = rng.random();
    }
    random_u64
}

fn handle_player_send(
    packet: PlayerPacket,
    player_id: u64,
    players: &mut HashMap<u64, Player>,
) -> Packet {
    match packet {
        PlayerPacket::PlayerPositionPacket(PlayerPosition { x, y, player_id }) => {
            players.get_mut(&player_id).unwrap().x = x;
            players.get_mut(&player_id).unwrap().y = y;
            return Packet::PlayerPacket(PlayerPacket::PlayerPositionPacket(PlayerPosition {
                player_id: player_id as u64,
                x,
                y,
            }));
        }
        PlayerPacket::PlayerDisconnectPacket(PlayerDisconnect { id }) => {
            return Packet::PlayerPacket(PlayerPacket::PlayerDisconnectPacket(PlayerDisconnect {
                id,
            }));
        }
        PlayerPacket::PlayerAnimationPacket(PlayerAnimation { id, animation_data }) => {
            players.get_mut(&player_id).unwrap().animation_data = Some(animation_data.clone());
            return Packet::PlayerPacket(PlayerPacket::PlayerAnimationPacket(PlayerAnimation {
                id,
                animation_data,
            }));
        }
        PlayerPacket::PlayerWelcomePacket(PlayerWelcome { player_id, x, y }) => {
            return Packet::PlayerPacket(PlayerPacket::PlayerWelcomePacket(PlayerWelcome {
                player_id,
                x,
                y,
            }));
        }
        PlayerPacket::PlayerLevelPacket(PlayerLevel { player_id, level }) => {
            players.get_mut(&player_id).unwrap().current_level = level.clone();
            return Packet::PlayerPacket(PlayerPacket::PlayerLevelPacket(PlayerLevel {
                player_id,
                level,
            }));
        }
    }
}

fn send_to_clients(packet: Packet, clients: &mut HashMap<u64, TcpStream>) {
    match packet {
        Packet::PlayerPacket(PlayerPacket::PlayerDisconnectPacket(PlayerDisconnect { id })) => {
            clients.remove(&id);
        }
        _ => (),
    }

    for (_, mut client) in clients.iter_mut() {
        serialize_and_send(&mut client, packet.clone());
    }
}

pub fn server() {
    // create a listener
    let listener = TcpListener::bind(LOCAL).expect("Failed to bind");
    listener
        .set_nonblocking(true)
        .expect("Failed to initialize non-blocking");

    let mut clients: HashMap<u64, TcpStream> = HashMap::new(); // uuid to tcp stream
    // intermediate step to handle client messages (so we dont need arc(mutex) on everything to pass between threads)
    let (server_sender, server_receiver) = mspc::channel::<ServerPacket>();

    let mut ip_to_uuid: HashMap<std::net::SocketAddr, u64> = HashMap::new();
    let mut uuid_to_ip: HashMap<u64, std::net::SocketAddr> = HashMap::new();
    let mut used_uuid = HashSet::new();

    let mut players: HashMap<u64, Player> = HashMap::new();

    loop {
        //std::thread::sleep(std::time::Duration::from_secs_f64(1./5.));
        // socket reading
        if let Ok((mut socket, addr)) = listener.accept() {
            println!("Client {} connected", addr);
            let server_sender = server_sender.clone();

            // add a new uuid for the client
            let uuid = new_client_id(&used_uuid);
            used_uuid.insert(uuid);
            ip_to_uuid.insert(addr, uuid);
            uuid_to_ip.insert(uuid, addr);

            // add player to the player list
            server_sender
                .send(ServerPacket::AddPlayer(addr))
                .expect("Failed adding client to player list");
            clients.insert(uuid, socket.try_clone().expect("Failed to clone client"));

            // send the client id to the client
            send_to_clients(Packet::ClientIDPacket(ClientID { id: uuid }), &mut clients);

            // read from the socket, new thread for each client
            thread::spawn(move || {
                loop {
                    match try_read_tcp(&mut socket) {
                        NetworkResult::Ok(buf) => {
                            match deserialize_to_packet(buf) {
                                // handling in the main thread
                                Some(packet) => server_sender
                                    .send(ServerPacket::ServerInternalPacket(ServerInternal {
                                        address: addr,
                                        packet,
                                    }))
                                    .expect("Failed to send internal server packet"),
                                _ => println!("Unknown packet"),
                            }
                        }
                        NetworkResult::WouldBlock => (),
                        NetworkResult::ConnectionLost => {
                            println!("Closing connection with: {}", addr);
                            // send packet that signals client disconnect
                            server_sender
                                .send(ServerPacket::RemovePlayer(addr))
                                .expect("Failed to send disconnect packet");
                            break;
                        }
                    }
                }
            });
        }

        // handle incoming packets
        if let Ok(packet) = server_receiver.try_recv() {
            match packet {
                ServerPacket::ServerInternalPacket(packet) => {
                    let (addr, packet) = (packet.address, packet.packet);
                    match packet {
                        Packet::PlayerPacket(packet) => {
                            let sender_uuid = ip_to_uuid.get(&addr).unwrap().clone();
                            let packet = handle_player_send(packet, sender_uuid, &mut players);
                            send_to_clients(packet, &mut clients);
                        }
                        _ => (),
                    }
                }
                ServerPacket::AddPlayer(addr) => {
                    let uuid = *ip_to_uuid.get(&addr).unwrap();
                    players.insert(uuid, Player::new(uuid));
                }
                ServerPacket::RemovePlayer(addr) => {
                    let uuid = *ip_to_uuid.get(&addr).unwrap();
                    //players.remove(&uuid);
                    let disconnect_packet = Packet::PlayerPacket(
                        PlayerPacket::PlayerDisconnectPacket(PlayerDisconnect { id: uuid as u64 }),
                    );
                    send_to_clients(disconnect_packet, &mut clients);
                }
            }
        }
    }
}
