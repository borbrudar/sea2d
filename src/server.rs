use crate::shared::{LOCAL,MSG_SIZE};
use std::io::{ErrorKind,Read,Write};
use std::net::TcpListener;
use std::sync::mpsc as mspc;
use std::thread;

pub fn server(){
    // create a listener
    let listener = TcpListener::bind(LOCAL).expect("Failed to bind");
    listener.set_nonblocking(true).expect("Failed to initialize non-blocking");

    let mut clients = vec![];
    let (tx,rx) = mspc::channel::<String>();   

    loop {
        // socket reading
        if let Ok((mut socket, addr)) = listener.accept() {
            println!("Client {} connected", addr);
            let tx: mspc::Sender<String> = tx.clone();
            let mut id_msg = ("CLIENT_ID: ".to_string() + &clients.len().to_string()).into_bytes();
            id_msg.resize(MSG_SIZE, 0);
            socket.write_all(&id_msg).map(|_| {}).ok();

            
            clients.push(socket.try_clone().expect("Failed to clone client"));
        
            // read from the socket, new thread for each client
            thread::spawn(move || loop {
                let mut buf = vec![0; MSG_SIZE];
                match socket.read_exact(&mut buf) {
                    Ok(_) => {
                        let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                        println!("{}: {:?}", addr, msg);
                        tx.send(msg).expect("Failed to send message to rx");
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
                let mut buf = msg.clone().into_bytes();
                buf.resize(MSG_SIZE, 0);
                client.write_all(&buf).map(|_| client).ok()
            }).collect::<Vec<_>>();
            
        }
    }
}

