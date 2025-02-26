use std::env;
use std::io::{ErrorKind,Read,Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc as mspc;
use std::thread;
use sdl2;

const LOCAL : &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn server(){
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

                thread::sleep(::std::time::Duration::from_millis(100));
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


fn client(){
    thread::spawn(|| {
        game_loop();
    });

    let mut client = TcpStream::connect(LOCAL).expect("Failed to connect");
    client.set_nonblocking(true).expect("Failed to initialize non-blocking client");

    let (tx,rx) = mspc::channel::<String>();

    thread::spawn(move || loop{
        let mut buf = vec![0; MSG_SIZE];
        match client.read_exact(&mut buf){
            Ok(_) => {
                let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let msg = String::from_utf8(msg).expect("Invalid utf8 message");
                println!("message recv {:?}", msg);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Connection lost");
                break;
            }
        };

        match rx.try_recv(){
            Ok(msg) => {
                let mut buf = msg.clone().into_bytes();
                buf.resize(MSG_SIZE, 0);
                client.write_all(&buf).expect("Writing to socket failed");
                println!("message sent {:?}", msg);
            },
            Err(mspc::TryRecvError::Empty) => (),
            Err(mspc::TryRecvError::Disconnected) => break,
        }
        thread::sleep(::std::time::Duration::from_millis(100));
    });

    println!("Write a message:");
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("Failed to read from stdin");
        let msg = buf.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {break}
    }
    println!("Bye bye!");
}



fn find_sdl_gl_driver() -> Option<u32> {
    for (index,item) in sdl2::render::drivers().enumerate(){
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn game_loop(){
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => {
                    break 'running
                },
                _ => {}
            }
        }
        canvas.clear();
        canvas.present();
    }
}

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