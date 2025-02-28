use serde::Serialize;

use crate::packet::{Packet, PacketInternal};
use crate::shared::*;
use crate::player::{Movement, Player, PlayerID, PlayerMovement, PlayerPacket, PlayerPosition, PlayerWelcome};

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

    let mut player = Player::new(1_000_000);
    let mut other_players : Vec<Player> = Vec::new();

    
    'running: loop {
        // event polling
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} | 
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::ESCAPE),..} => {
                    break 'running
                },
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::UP),..} => {
                    player.y -= 15;
                    let send = Packet::PlayerPacket(PlayerPacket::PlayerMovementPacket(PlayerMovement{mov : Movement::Down}));
                    tx.send(send).unwrap();
                },
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::DOWN),..} => {
                    player.y += 15;
                    let send = Packet::PlayerPacket(PlayerPacket::PlayerMovementPacket(PlayerMovement{mov : Movement::Up}));
                    tx.send(send).unwrap();
                },
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::LEFT),..} => {
                    player.x -= 15;
                    let send = Packet::PlayerPacket(PlayerPacket::PlayerMovementPacket(PlayerMovement{mov : Movement::Left}));
                    tx.send(send).unwrap();
                },
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::RIGHT),..} => {
                    player.x += 15;
                    let send = Packet::PlayerPacket(PlayerPacket::PlayerMovementPacket(PlayerMovement{mov : Movement::Right}));
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
                        if player.id == 1_000_000{
                            player.id = id.id;
                        }
                        if player.id == 0{
                            player.color = (255,0,0);
                        }else {
                            player.color = (0,0,255);
                        }
                    },
                    None => println!("Not an id")
                }

                match msg.try_deserialize::<PlayerPosition>(){
                    Some(pos) => {
                        println!("Got a position :{:?}", pos);
                        for i in 0..other_players.len(){
                            if other_players[i].id == pos.player_id {
                                other_players[i].x = pos.x;
                                other_players[i].y = pos.y;
                            }
                        }
                    },
                    None => println!("Not a movement")
                }

                match msg.try_deserialize::<PlayerWelcome>(){
                    Some( welc) =>{
                        println!("Got a welcome packet");
                        // if self or already received return
                        if welc.player_id == player.id {()} 
                        for i in 0..other_players.len(){
                            if other_players[i].id == welc.player_id {()}
                        }
                        // else add to vector of other players
                        let mut temp = Player::new(welc.player_id);
                        temp.x = welc.x; temp.y = welc.y;
                        other_players.push(temp);
                    },
                    None => println!("Not a welcome packet")
                }
            },
            Err(mspc::TryRecvError::Empty) => (),
            Err(mspc::TryRecvError::Disconnected) => break,
        }

        // drawing
        canvas.clear();

        // draw other player
        for i in 0..other_players.len(){
            other_players[i].draw(&mut canvas);
        }
        // draw self
        player.draw(&mut canvas);
        
        
        // clear screen
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
        canvas.present();
       // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32/60));
    }
}
