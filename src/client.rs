use crate::animated_texture::AnimatedTexture;
use crate::packet::{ClientID, Packet, PacketInternal};
use crate::player::Player;
use crate::shared::*;
use crate::player_packets::*;

use std::io::{ErrorKind,Read,Write};
use std::net::TcpStream;
use std::sync::mpsc as mspc;
use std::thread;
use sdl2::image::{self};
use std::collections::HashMap;
use crate::texture_data::TextureData;
use sdl2::render::Texture;
use crate::level::Level;
use crate::camera::Camera;


pub fn client(address : &str ) {
    let mut client = TcpStream::connect(address).expect("Failed to connect");
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
                println!("Connection lost client2");
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
            Err(mspc::TryRecvError::Disconnected) => {
                println!("Connection lost client");
                break;
            }
        }
    });
    
    game_loop(tx,rx2);
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


fn game_loop(tx : mspc::Sender<Packet>, rx : mspc::Receiver<PacketInternal>) {
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
    
    let mut other_players : HashMap<u64,Player> = HashMap::new();
    
    image::init(image::InitFlag::PNG | image::InitFlag::JPG).unwrap();
    // let texture = load_texture_from_file(&sdl_context, "path/to/your/image.png")?;
    let texture_creator = canvas.texture_creator();
    let mut texture_map: HashMap<String, Texture> = HashMap::new();
    let mut player = Player::new(1_000_000);
    player.texture_data = Some(TextureData::new("resources/textures/test.png".to_string()));
    player.texture_data.as_mut().unwrap().load_texture(&texture_creator, &mut texture_map);

    player.animation_data = Some(AnimatedTexture::new(1.0/4.));
    player.animation_data.as_mut().unwrap().load_animation("resources/player_animation/woman.png".to_string(),0,0,626/4,313/2,4,
    &texture_creator,&mut texture_map);
                        

    let mut level = Level::new();
    level.load_from_file("resources/levels/level1.png".to_string(),&texture_creator,&mut texture_map);
    let mut camera = Camera::new(0,0,SCREEN_WIDTH,SCREEN_HEIGHT);

    let mut current_time = std::time::Instant::now();
    let time_step = 1.0/60.0;
    'running: loop {
        // event polling
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} | 
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::ESCAPE),..} => {
                    break 'running
                },
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::UP),..} => {
                    camera.move_camera(0, -15);
                    player.y -= 15;
                    let send = Packet::PlayerPacket(PlayerPacket::PlayerMovementPacket(PlayerMovement{mov : Movement::Down}));
                    tx.send(send).unwrap();
                },
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::DOWN),..} => {
                    camera.move_camera(0, 15);
                    player.y += 15;
                    let send = Packet::PlayerPacket(PlayerPacket::PlayerMovementPacket(PlayerMovement{mov : Movement::Up}));
                    tx.send(send).unwrap();
                },
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::LEFT),..} => {
                    camera.move_camera(-15, 0);
                    player.x -= 15;
                    let send = Packet::PlayerPacket(PlayerPacket::PlayerMovementPacket(PlayerMovement{mov : Movement::Left}));
                    tx.send(send).unwrap();
                },
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::RIGHT),..} => {
                    player.x += 15;
                    camera.move_camera(15, 0);
                    let send = Packet::PlayerPacket(PlayerPacket::PlayerMovementPacket(PlayerMovement{mov : Movement::Right}));
                    tx.send(send).unwrap();
                },
                sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::C), .. } => {
                    player.texture_data = Some(TextureData::new("resources/textures/lmao.png".to_string()));
                    player.texture_data.as_mut().unwrap().load_texture(&texture_creator, &mut texture_map);
                    let send = Packet::PlayerPacket(PlayerPacket::PlayerTextureDataPacket(PlayerTextureData{texture_data : player.texture_data.clone().unwrap(), id : player.id}));
                    tx.send(send).unwrap();
                }
                _ => {}
            }
        }


        // time handling
        let new_time = std::time::Instant::now();
        let mut frame_time = new_time - current_time;
        current_time = new_time;

        // update
        while frame_time > std::time::Duration::from_secs_f64(0.0){
            let delta_time = f64::min(frame_time.as_secs_f64(), time_step);
            
            if !player.animation_data.is_none(){
                player.animation_data.as_mut().unwrap().update(delta_time);
            }
            for (_,other_player) in &mut other_players{
                if !other_player.animation_data.is_none(){
                    other_player.animation_data.as_mut().unwrap().update(delta_time);
                }
            }

            frame_time -= std::time::Duration::from_secs_f64(delta_time);
        }

        // drawing
        canvas.clear();
        // draw level
        level.draw(&mut canvas,&texture_map,&camera);
        
        //draw other player
        for (_,other_player) in &mut other_players{
            other_player.draw(&mut canvas,&texture_map,&camera);
        }
        // draw self
        player.draw(&mut canvas,&texture_map,&camera);
        
        // Draw self (player)
        // clear screen
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
        canvas.present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32/60));
        
        match rx.try_recv(){
            Ok(msg) => {
                match msg.try_deserialize::<ClientID>(){
                    Some(id) => {
                        println!("Got an id :{}",id.id);
                        if player.id == 1_000_000{
                            player.id = id.id;
                        }
                        tx.send(Packet::PlayerPacket(PlayerPacket::PlayerTextureDataPacket(
                            PlayerTextureData{texture_data : player.texture_data.clone().unwrap(),id : player.id}))).unwrap();

                        let data = PlayerAnimation{id : player.id, animation_data : player.animation_data.clone().unwrap()};
                        println!("Sending animation packet");
                        tx.send(Packet::PlayerPacket(PlayerPacket::PlayerAnimationPacket(data))).unwrap();
                        
                    },
                    None => ()
                }

                match msg.try_deserialize::<PlayerPosition>(){
                    Some(pos) => {
                        println!("Got a position :{:?}", pos);
                        if let Some(other_player) = other_players.get_mut(&pos.player_id) {
                            other_player.x = pos.x;
                            other_player.y = pos.y;
                        }
                    },
                    None => ()  
                }

                match msg.try_deserialize::<PlayerWelcome>(){
                    Some( welc) =>{
                        println!("Got a welcome packet");
                        // if self or already received return
                        let found = other_players.contains_key(&welc.player_id) || welc.player_id == player.id;
                        if !found {
                            let mut temp = Player::new(welc.player_id);
                            temp.x = welc.x;
                            temp.y = welc.y;
                            temp.texture_data = welc.texture_data;
                
                            if let Some(mut texture_data) = temp.texture_data.clone() {
                                texture_data.load_texture(&texture_creator, &mut texture_map);
                            } else {
                                println!("No texture data");
                            }
                            other_players.insert(temp.id, temp);
                        }
                    },
                    None => ()
                }

                match msg.try_deserialize::<PlayerTextureData>(){
                    Some(texture_data) => {
                        println!("Got a texture data packet");
                        if let Some(other_player) = other_players.get_mut(&texture_data.id) {
                            other_player.texture_data = Some(texture_data.texture_data.clone());
                            match other_player.texture_data.clone() {
                                Some(mut texture_data) => {
                                    texture_data.load_texture(&texture_creator, &mut texture_map);
                                },
                                None => println!("No texture data")
                            }
                        }
                    },
                    None => ()
                }

                match msg.try_deserialize::<PlayerDisconnect>(){
                    Some(disconnected) => {
                        println!("Got a disconnect packet");
                        other_players.remove(&disconnected.id);
                    },
                    None => ()
                }

                match msg.try_deserialize::<PlayerAnimation>(){
                    Some(animation) => {
                        println!("Got an animation packet");
                        if let Some(other_player) = other_players.get_mut(&animation.id) {
                            println!("Processed animation packet");
                            other_player.animation_data = Some(animation.animation_data.clone());
                            println!("Received animation data {:?}", &other_player.animation_data);
                            other_player.animation_data.as_mut().unwrap().load_animation(animation.animation_data.frames[0].path.clone(), 0, 0, 626/4, 313/2, 4, &texture_creator, &mut texture_map);
                            println!("Received animation data2 {:?}", &other_player.animation_data);
                        }
                    },
                    None => ()
                }
            },
            Err(mspc::TryRecvError::Empty) => (),
            Err(mspc::TryRecvError::Disconnected) => break,
        }
    }
}
