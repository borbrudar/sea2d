use serde::Serialize;

use crate::packet::{Packet,PlayerMovement,Movement,PacketInternal};
use crate::shared::*;

use std::io::{ErrorKind,Read,Write};
use std::net::TcpStream;
use std::sync::mpsc as mspc;
use std::thread;


pub fn client(){
    let mut client = TcpStream::connect(LOCAL).expect("Failed to connect");
    client.set_nonblocking(true).expect("Failed to initialize non-blocking client");
    
    let (tx,rx) = mspc::channel::<Packet>(); // send from game thread to connection thread
    let (tx2 , rx2) = mspc::channel::<String>(); // send to game thread from connection thread

    thread::spawn(move || loop{
        let mut buf = vec![0; MSG_SIZE];
        // read from server
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
        // send to server
        match rx.try_recv(){
            Ok(msg) => {
                let mut packet_int = PacketInternal::new(msg.clone()).unwrap();
                let mut buf : Vec<u8> = vec![0;0];
                for i in 0..8 {
                    buf.push(((packet_int.type_id>>(i*8))&((1<<8)-1)) as u8);
                }
                buf.append(&mut packet_int.data);
                
                if buf.len() > MSG_SIZE {
                    panic!("Message length exceeded");
                }
                else{
                    buf.append(&mut vec![0;MSG_SIZE - buf.len()]);
                }

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


fn game_loop(tx : mspc::Sender<Packet>, rx : mspc::Receiver<String>){
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
    let mut px2 = -10000;
    let mut py2 = -10000;
    let mut color = (255,255,255);
    let mut col2 = (0,0,0);
    let psize: u32 = 40;
    let mut id : String = "-1".to_string();

    'running: loop {
        // event polling
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} | 
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::ESCAPE),..} => {
                    break 'running
                },
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::UP),..} => {
                    py -= 15;
                    let send = Packet::PlayerMovementPacket(PlayerMovement{mov: Movement::Down});
                    tx.send(send).unwrap();
                },
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::DOWN),..} => {
                    py += 15;
                    let send = Packet::PlayerMovementPacket(PlayerMovement{mov: Movement::Up});
                    tx.send(send).unwrap();
                },
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::LEFT),..} => {
                    px -= 15;
                    let send = Packet::PlayerMovementPacket(PlayerMovement{mov: Movement::Left});
                    tx.send(send).unwrap();
                },
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::RIGHT),..} => {
                    px += 15;
                    let send = Packet::PlayerMovementPacket(PlayerMovement{mov: Movement::Right});
                    tx.send(send).unwrap();
                },
                _ => {}
            }
        }

        match rx.try_recv(){
            Ok(msg) => {
                let prt = msg.clone();
                
                let msg = msg.split(' ').collect::<Vec<&str>>();

                if msg.len() <= 1 {continue}
                if msg[0] != id && msg.len()>=3{
                    px2 = msg[1].parse().unwrap();
                    py2 = msg[2].parse().unwrap();
                }
                if msg[0] == "CLIENT_ID:"  {
                    if id == "-1"{
                        id = msg[1].to_string();
                        println!("Got an id :{}",&id);
                        if id == "0"{
                            color = (255,0,0);
                            col2 = (0,0,255);
                        }else {
                            color = (0,0,255);
                            col2 = (255,0,0);
                        }
                    }
                }
                
                println!("{}",prt);
            },
            Err(mspc::TryRecvError::Empty) => (),
            Err(mspc::TryRecvError::Disconnected) => break,
        }

        // drawing
        canvas.clear();

        // draw other player
        canvas.set_draw_color(sdl2::pixels::Color::RGB(col2.0,col2.1,col2.2));
        canvas.fill_rect(sdl2::rect::Rect::new(px2,py2,psize,psize)).unwrap();

        // draw self
        canvas.set_draw_color(sdl2::pixels::Color::RGB(color.0,color.1,color.2));
        canvas.fill_rect(sdl2::rect::Rect::new(px,py,psize,psize)).unwrap();
        
        
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
        canvas.present();
       // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32/60));
    }
}
