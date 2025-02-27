use serde::Serialize;

use crate::packet::{Movement, Packet, PacketInternal, PlayerID, PlayerMovement, PlayerPosition};
use crate::shared::*;

use std::io::{ErrorKind,Read,Write};
use std::net::TcpStream;
use std::sync::mpsc as mspc;
use std::thread;


pub fn client(){
    let mut client = TcpStream::connect(LOCAL).expect("Failed to connect");
    client.set_nonblocking(true).expect("Failed to initialize non-blocking client");
    
    let (tx,rx) = mspc::channel::<Packet>(); // send from game thread to connection thread
    let (tx2 , rx2) = mspc::channel::<PacketInternal>(); // send to game thread from connection thread

    thread::spawn(move || loop{
        let mut buf = vec![0; MSG_SIZE];
        // read from server
        match client.read_exact(&mut buf){
            Ok(_) => {
                let received: Vec<u8> = buf.into_iter().collect::<Vec<_>>();
                let packet_int  = bincode::deserialize(&received);
                match packet_int{
                    Ok(packet_int) =>
                     tx2.send(packet_int).expect("Failed to send packet to game thread"),
                    Err(err) => println!("Failed to deserialize packet {:?}",err)
                }
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
                let packet_int = PacketInternal::new(msg.clone()).unwrap();
                let mut send = bincode::serialize(&packet_int).unwrap();
                
                if send.len() > MSG_SIZE {
                    panic!("Message length exceeded");
                }
                else{
                    send.append(&mut vec![0;MSG_SIZE - send.len()]);
                }

                client.write_all(&send).expect("Writing to socket failed");
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


fn game_loop(tx : mspc::Sender<Packet>, rx : mspc::Receiver<PacketInternal>){
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
    let mut my_id : u64 = 1000000000;

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
                match msg.try_deserialize::<PlayerID>(){
                    Some(id) => {
                        println!("Got an id :{}",id.id);
                        if my_id == 1000000000{
                            my_id = id.id;
                        }
                        if my_id == 0{
                            color = (255,0,0);
                            col2 = (0,0,255);
                        }else {
                            color = (0,0,255);
                            col2 = (255,0,0);
                        }
                    },
                    None => println!("Not an id")
                }

                match msg.try_deserialize::<PlayerPosition>(){
                    Some(mov) => {
                        println!("Got a position :{:?}", mov);
                        if mov.player_id != my_id{
                            px2 = mov.x;
                            py2 = mov.y;
                        }
                    },
                    None => println!("Not a movement")
                }
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
