mod aabb;
mod animated_texture;
mod button;
mod camera;
mod client;
mod enemy;
mod game;
mod hud;
mod level;
mod networking;
mod packet;
mod player;
mod player_packets;
mod point;
mod server;
mod shared;
mod texture_data;
mod tile;
mod tile_type;
use crate::client::client;
use crate::server::server;
use crate::shared::CLIENT_LOCAL;
use std::env;
use std::thread;
#[cfg(test)]
mod tests;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() >= 2 && args[1] == "client" {
        if args.len() >= 3 {
            client(&args[2]);
        } else {
            client(CLIENT_LOCAL);
        }
    } else if args.len() >= 2 && args[1] == "server" {
        println!("Running server on localhost:6000");
        server();
    } else {
        println!("Running server-client on localhost:6000");
        let _server = thread::spawn(|| {
            server();
        });
        let client = thread::spawn(|| {
            client(CLIENT_LOCAL);
        });
        client.join().unwrap();
    }
}
