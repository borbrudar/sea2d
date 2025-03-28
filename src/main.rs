mod server;
mod client;
mod shared;
mod packet;
mod player;
mod texture_data;
mod player_packets;
mod tile;
mod level;
mod camera;
mod animated_texture;
mod tile_type;
mod aabb;
mod hud;
mod button;
mod game;
mod networking;
mod enemy;
mod point;
use crate::server::server;
use crate::client::client;
use std::env;
use std::thread;
use crate::shared::CLIENT_LOCAL;
#[cfg(test)]
mod tests;


fn main() {
    let args = env::args().collect::<Vec<String>>();
   
    if args.len() >= 2 && args[1] == "client"{ 
        if args.len() >= 3{
            client(&args[2]);
        } else {
            client(CLIENT_LOCAL);
        }
    }else if args.len() >= 2 && args[1] == "server"{
        println!("Running server on localhost:6000");
        server();
    }
    else {
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