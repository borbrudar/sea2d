mod game;
mod hud;
mod level;
mod networking;
mod player;
mod wfc;
use crate::networking::{client::client, server::server, shared::CLIENT_LOCAL};
use crate::wfc::run_wfc;
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
    } else if args.contains(&"--wfc".to_string()) {
        return run_wfc();
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
