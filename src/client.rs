use crate::shared::*;

use std::io::{ErrorKind,Read,Write};
use std::net::TcpStream;
use std::sync::mpsc as mspc;
use std::thread;
use std::time::Duration;


pub fn client(){
    let mut client = TcpStream::connect(LOCAL).expect("Failed to connect");
    client.set_nonblocking(true).expect("Failed to initialize non-blocking client");
    
    let (tx,rx) = mspc::channel::<String>(); // send from game thread to connection thread
    let (tx2 , rx2) = mspc::channel::<String>(); // send to game thread from connection thread

    thread::spawn(move || loop{
        let mut buf = vec![0; MSG_SIZE];
        match client.read_exact(&mut buf){
            Ok(_) => {
                let msg = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let msg = String::from_utf8(msg).expect("Invalid utf8 message");
               // println!("message recv {:?}", msg);
                tx2.send(msg).expect("Failed to send msg to game thread");
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
    //thread::sleep(::std::time::Duration::from_millis(100));
    });
    
    println!("Write a message:");
    game_loop(tx,rx2);

    /*
    let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("Failed to read from stdin");
        let msg = buf.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {break}
     */
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


fn game_loop(tx : mspc::Sender<String>, rx : mspc::Receiver<String>){
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("sea2d", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut px: i32 = (SCREEN_WIDTH as i32)/2;
    let mut py: i32 = (SCREEN_HEIGHT as i32)/2;
    let psize: u32 = 40;

    'running: loop {
        // event polling
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} | 
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::ESCAPE),..} => {
                    break 'running
                },
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::UP),..} => {
                    tx.send("UP".to_string()).unwrap();
                 //   py -= 15;
                },
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::DOWN),..} => {
                    tx.send("DOWN".to_string()).unwrap();
                 //   py += 15;
                },
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::LEFT),..} => {
                    tx.send("LEFT".to_string()).unwrap();
                    //px -= 15;
                },
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::RIGHT),..} => {
                    tx.send("RIGHT".to_string()).unwrap();
                   // px += 15;
                },
                _ => {}
            }
        }

        match rx.try_recv(){
            Ok(msg) => {
                if msg == "UP" {
                    py -= 15;
                } else if msg == "DOWN" {
                    py += 15;
                } else if msg == "LEFT" {
                    px -= 15;
                } else if msg == "RIGHT" {
                    px += 15;
                }
            },
            Err(mspc::TryRecvError::Empty) => (),
            Err(mspc::TryRecvError::Disconnected) => break,
        }

        // drawing
        canvas.clear();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255,255,255));
        canvas.fill_rect(sdl2::rect::Rect::new(px,py,psize,psize)).unwrap();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
        canvas.present();
       // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32/60));
    }
}
