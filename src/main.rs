mod server;
mod client;
mod shared;
mod packet;
mod player;
mod texture_data;
use crate::server::server;
use crate::client::client;
use std::env;
use std::thread;


fn main() {
    let args = env::args().collect::<Vec<String>>();
   
    if args.len() >= 2 && args[1] == "client"{ 
        println!("Running client on localhost:6000");
        client();
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
            client();
        });
        client.join().unwrap();
    }   
}