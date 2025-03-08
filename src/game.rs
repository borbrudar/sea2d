use crate::animated_texture::AnimatedTexture;
use crate::packet::{ClientID, Packet, PacketInternal};
use crate::player::Player;
use crate::shared::*;
use crate::player_packets::*;

use std::sync::mpsc as mspc;
use sdl2::image::{self};
use sdl2::pixels::Color;
use std::collections::HashMap;
use crate::texture_data::TextureData;
use sdl2::render::Texture;
use crate::level::Level;
use crate::camera::Camera;
use crate::hud::Hud;
use crate::animated_texture::AnimationType;


pub struct Game{
    packet_receiver : mspc::Receiver<PacketInternal>,
    packet_sender : mspc::Sender<Packet>,
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index,item) in sdl2::render::drivers().enumerate(){
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

impl Game{
    pub fn new(packet_sender : mspc::Sender<Packet>, packet_receiver : mspc::Receiver<PacketInternal>) -> Game{
        Game{
            packet_receiver,
            packet_sender,
        }
    }

    fn handle_receive<'a>(&self, player : &mut Player, other_players : &mut HashMap<u64,Player>, texture_creator : &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>, texture_map : &mut HashMap<String,Texture<'a>>) -> Result<(),&'static str>{
        match self.packet_receiver.try_recv(){
            Ok(msg) => {
                match msg.try_deserialize::<ClientID>(){
                    Some(id) => {
                        println!("Got an id :{}",id.id);
                        if player.id == 1_000_000{
                            player.id = id.id;
                        }
                        self.packet_sender.send(Packet::PlayerPacket(PlayerPacket::PlayerTextureDataPacket(
                            PlayerTextureData{texture_data : player.texture_data.clone().unwrap(),id : player.id}))).unwrap();
    
                        let data = PlayerAnimation{id : player.id, animation_data : player.animation_data.clone().unwrap()};
                        println!("Sending animation packet");
                        self.packet_sender.send(Packet::PlayerPacket(PlayerPacket::PlayerAnimationPacket(data))).unwrap();
                        
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
                                texture_data.load_texture(&texture_creator, texture_map);
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
                                    texture_data.load_texture(&texture_creator, texture_map);
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
                            other_player.animation_data.as_mut().unwrap().load_animation(animation.animation_data.frames[0].path.clone(), 0, 0, 16, 16, 3, &texture_creator, texture_map);
                            println!("Received animation data2 {:?}", &other_player.animation_data);
                        }
                    },
                    None => ()
                }
            },
            Err(mspc::TryRecvError::Empty) => (),
            Err(mspc::TryRecvError::Disconnected) => Err("Disconnected").unwrap(),
        }
        Ok(())
    }
    


    // main game loop
    pub fn run(&mut self){
        // initalize sdl2 stuff
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
        let texture_creator = canvas.texture_creator();
        let mut texture_map: HashMap<String, Texture> = HashMap::new();
        let mut player = Player::new(1_000_000);
        player.texture_data = Some(TextureData::new("resources/textures/test.png".to_string()));
        player.texture_data.as_mut().unwrap().load_texture(&texture_creator, &mut texture_map);

        player.animation_data = Some(AnimatedTexture::new(1.0/6.));
        player.animation_data.as_mut().unwrap().load_animation("resources/player_animation/player.png".to_string(),0,0,16,16,3, 
        &texture_creator,&mut texture_map);
        player.animation_data.as_mut().unwrap().animation_type = AnimationType::PingPong;                    


        let mut level = Level::new();
        level.load_from_file("resources/levels/level1_1.png".to_string(),&texture_creator,&mut texture_map);
        let mut camera = Camera::new(0,0,SCREEN_WIDTH,SCREEN_HEIGHT);
        camera.x = player.x - SCREEN_WIDTH as i32/2;
        camera.y = player.y - SCREEN_HEIGHT as i32/2;

        // hud
        let hud = Hud::new();
        let mut draw_hitboxes = false;

        let mut current_time = std::time::Instant::now();
        let time_step = 1.0/60.0;
        
        'running: loop {
            // event polling
            for event in event_pump.poll_iter() {
                player.on_event(event.clone(), &self.packet_sender, &level, &mut camera);
                match event {
                    sdl2::event::Event::Quit {..} | 
                    sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::ESCAPE),..} => {
                        break 'running
                    },
                    sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::C), .. } => {
                        player.texture_data = Some(TextureData::new("resources/textures/lmao.png".to_string()));
                        player.texture_data.as_mut().unwrap().load_texture(&texture_creator, &mut texture_map);
                        let send = Packet::PlayerPacket(PlayerPacket::PlayerTextureDataPacket(PlayerTextureData{texture_data : player.texture_data.clone().unwrap(), id : player.id}));
                        self.packet_sender.send(send).unwrap();
                    }
                    sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::H), .. } => {
                        draw_hitboxes = !draw_hitboxes;
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
            if draw_hitboxes {
                level.draw_hitboxes(&mut canvas,&camera);
            }
            //draw other player
            for (_,other_player) in &mut other_players{
                other_player.draw(&mut canvas,&texture_map,&camera);
            }
            // draw self
            player.draw(&mut canvas,&texture_map,&camera);
            let player_hitbox_color = if player.colliding {Color::RED} else {Color::GREEN};
            
            if draw_hitboxes{
                player.hitbox.draw(&mut canvas,player_hitbox_color,&camera);
            }
            
            hud.draw(&mut canvas);
    
            // Draw self (player)
            // clear screen
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
            canvas.present();
    
    
            // receive
            self.handle_receive(&mut player, &mut other_players, &texture_creator, &mut texture_map).unwrap();   

        }
    }
}