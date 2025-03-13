use crate::animated_texture::AnimatedTexture;
use crate::packet::Packet;
use crate::player::Player;
use crate::shared::*;
use crate::player_packets::*;

use std::sync::mpsc as mspc;
use sdl2::image::{self};
use sdl2::pixels::Color;
use sdl2::rect;
use std::collections::HashMap;
use sdl2::render::Texture;
use crate::level::Level;
use crate::camera::Camera;
use crate::hud::Hud;
use crate::animated_texture::AnimationType;


pub enum GameState{
    Running,
    Paused
}

pub struct Game{
    packet_receiver : mspc::Receiver<Packet>,
    packet_sender : mspc::Sender<Packet>,
    game_state : GameState
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
    pub fn new(packet_sender : mspc::Sender<Packet>, packet_receiver : mspc::Receiver<Packet>) -> Game{
        Game{
            packet_receiver,
            packet_sender,
            game_state : GameState::Running
        }
    }

    fn handle_receive<'a>(&self, player : &mut Player, other_players : &mut HashMap<u64,Player>, texture_creator : &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>, texture_map : &mut HashMap<String,Texture<'a>>){
        match self.packet_receiver.try_recv(){
            Ok(packet) => {
                match packet {
                    Packet::PlayerPacket(player_packet) => {
                        match player_packet {
                            PlayerPacket::PlayerPositionPacket(pos) => {
                                //println!("Got a position :{:?}", pos);
                                if let Some(other_player) = other_players.get_mut(&pos.player_id) {
                                    other_player.x = pos.x;
                                    other_player.y = pos.y;
                                }
                            },
                            PlayerPacket::PlayerWelcomePacket(welc) => {
                                //println!("Got a welcome packet");
                                // if self or already received return
                                let found = other_players.contains_key(&welc.player_id) || welc.player_id == player.id;
                                if !found {
                                    let mut temp = Player::new(welc.player_id);
                                    temp.x = welc.x;
                                    temp.y = welc.y;
                
                                    other_players.insert(temp.id, temp);
                                }
                            },
                            PlayerPacket::PlayerDisconnectPacket(disconnected) => {
                                //println!("Got a disconnect packet");
                                other_players.remove(&disconnected.id);
                            },
                            PlayerPacket::PlayerAnimationPacket(animation) => {
                                //println!("Got an animation packet");
                                if let Some(other_player) = other_players.get_mut(&animation.id) {
                                    other_player.animation_data = Some(animation.animation_data.clone());
                                    other_player.animation_data.as_mut().unwrap().load_animation(animation.animation_data.frames[0].path.clone(), 0, 0, 16, 16, 3, &texture_creator, texture_map);
                                }
                            },
                        }
                    },
                    Packet::ClientIDPacket(id) =>{
                        println!("Got an id :{}",id.id);
                        if player.id == 1_000_000{
                            player.id = id.id;
                        }
                        let data = PlayerWelcome{player_id : player.id, x : player.x, y : player.y};
                        self.packet_sender.send(Packet::PlayerPacket(PlayerPacket::PlayerWelcomePacket(data))).unwrap();
                        let data = PlayerAnimation{id : player.id, animation_data : player.animation_data.clone().unwrap()};
                        self.packet_sender.send(Packet::PlayerPacket(PlayerPacket::PlayerAnimationPacket(data))).unwrap();
                    }
                }
            },
            Err(mspc::TryRecvError::Empty) =>(),
            Err(mspc::TryRecvError::Disconnected) => panic!("Disconnected"),
        }     
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

        let viewport = rect::Rect::new(0,0,SCREEN_WIDTH,SCREEN_HEIGHT);
        canvas.set_viewport(viewport);
        let mut event_pump = sdl_context.event_pump().unwrap();
        
        
        let mut other_players : HashMap<u64,Player> = HashMap::new();
        
        // texture setup
        image::init(image::InitFlag::PNG | image::InitFlag::JPG).unwrap();
        let texture_creator = canvas.texture_creator();
        let mut texture_map: HashMap<String, Texture> = HashMap::new();
        
        // level loading
        let mut level = Level::new();
        level.load_from_file("resources/levels/level1_1.png".to_string(),&texture_creator,&mut texture_map);
        
        // player setup
        let mut player = Player::new(1_000_000);
        player.animation_data = Some(AnimatedTexture::new(1.0/6.));
        player.animation_data.as_mut().unwrap().load_animation("resources/player_animation/player.png".to_string(),0,0,16,16,3, 
        &texture_creator,&mut texture_map);
        player.animation_data.as_mut().unwrap().animation_type = AnimationType::PingPong;                    
        player.x = level.player_spawn.0 as f64;
        player.y = level.player_spawn.1 as f64;
        player.hitbox.x = player.x + 10.;
        player.hitbox.y = player.y + 15.;

        // camera init
        let mut camera = Camera::new(player.x + (player.size as i32/2 - SCREEN_WIDTH as i32/2) as f64,
        player.y + (player.size as i32/2 - SCREEN_HEIGHT as i32/2) as f64, SCREEN_WIDTH,SCREEN_HEIGHT);

        // hud
        let hud = Hud::new();
        let mut draw_hitboxes = false;

        let mut current_time = std::time::Instant::now();
        let time_step = 1.0/60.0;
        
        'running: loop {
            // event polling
            for event in event_pump.poll_iter() {
                match self.game_state {
                    GameState::Running => player.on_event(&event),
                    GameState::Paused => player.reset_velocity(),
                }
                //camera.handle_zoom(&event);
                match event {
                    sdl2::event::Event::Quit {..} | 
                    sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::ESCAPE),..} => {
                        break 'running
                    },
                    sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::H), .. } => {
                        draw_hitboxes = !draw_hitboxes;
                    }
                    sdl2::event::Event::KeyDown { keycode : Some(sdl2::keyboard::Keycode::P), ..} =>{
                        match self.game_state {
                            GameState::Paused => self.game_state = GameState::Running,
                            GameState::Running => self.game_state = GameState::Paused,
                        }
                    }
                    _ => {}
                }
            }

            // check if we need to load a new level
            if let Some(exit) = player.reached_end.clone(){
                level.load_from_file(exit.next_level.clone(),&texture_creator,&mut texture_map);
                player.x = level.player_spawn.0 as f64;
                player.y = level.player_spawn.1 as f64;
                player.hitbox.x = player.x + 10.0;
                player.hitbox.y = player.y + 15.0;
                camera.x = player.x + (player.size as i32/2 - SCREEN_WIDTH as i32/2) as f64;
                camera.y = player.y + (player.size as i32/2 - SCREEN_HEIGHT as i32/2) as f64;
                player.reached_end = None;
            }
    
    
            // time handling
            let new_time = std::time::Instant::now();
            let mut frame_time = new_time - current_time;
            current_time = new_time;
    
            // update
            while frame_time > std::time::Duration::from_secs_f64(0.0){
                let delta_time = f64::min(frame_time.as_secs_f64(), time_step);
                
                match self.game_state {
                    GameState::Running => {
                        player.update(delta_time, &self.packet_sender, &level, &mut camera);
                        for (_,other_player) in &mut other_players{
                            if !other_player.animation_data.is_none(){
                                other_player.animation_data.as_mut().unwrap().update(delta_time);
                            }
                        }
                    }
                    _ => ()
                }
    
                frame_time -= std::time::Duration::from_secs_f64(delta_time);
            }
    
            // drawing
            canvas.clear();

            //let viewport = rect::Rect::new(0,0,SCREEN_WIDTH,SCREEN_HEIGHT);
            let viewport = rect::Rect::new(0,0, camera.width, camera.height);
            canvas.set_viewport(viewport);

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

            // clear screen
            match self.game_state{
                GameState::Paused => {
                    canvas.set_draw_color(sdl2::pixels::Color::RGBA(00,00,255,150));
                    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
                    canvas.fill_rect(rect::Rect::new(0,0,SCREEN_WIDTH,SCREEN_HEIGHT)).unwrap();
                },
                _ => ()
            }

            canvas.present();
    
    
            // receive
            self.handle_receive(&mut player, &mut other_players, &texture_creator, &mut texture_map);   

        }
    }
}