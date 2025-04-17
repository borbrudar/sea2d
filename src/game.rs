use crate::animated_texture::AnimatedTexture;
use crate::enemy::Enemy;
use crate::packet::Packet;
use crate::player::Player;
use crate::player_packets::*;
use crate::shared::*;

use crate::animated_texture::AnimationType;
use crate::button::Button;
use crate::camera::Camera;
use crate::hud::Hud;
use crate::level::Level;
use sdl2::audio::AudioDevice;
use sdl2::image::{self};
use sdl2::mixer;
use sdl2::mixer::Chunk;
use sdl2::mixer::Music;
use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureQuery;
use sdl2::ttf;
use std::collections::HashMap;
use std::sync::mpsc as mspc;

//lara
use std::cell::RefCell;
use std::rc::Rc;

pub enum GameState {
    Running,
    Paused,
    GameOver,
}

pub struct Game {
    packet_receiver: mspc::Receiver<Packet>,
    packet_sender: mspc::Sender<Packet>,
    game_state: GameState,
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

impl Game {
    pub fn new(
        packet_sender: mspc::Sender<Packet>,
        packet_receiver: mspc::Receiver<Packet>,
    ) -> Game {
        Game {
            packet_receiver,
            packet_sender,
            game_state: GameState::Running,
        }
    }

    fn handle_receive<'a>(
        &self,
        player: &mut Player,
        other_players: &mut HashMap<u64, Player>,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        texture_map: &mut HashMap<String, Texture<'a>>,
    ) {
        match self.packet_receiver.try_recv() {
            Ok(packet) => {
                //println!("Lmao");
                match packet {
                    Packet::PlayerPacket(player_packet) => {
                        match player_packet {
                            PlayerPacket::PlayerPositionPacket(pos) => {
                                //println!("Got fake positoin'");
                                if let Some(other_player) = other_players.get_mut(&pos.player_id) {
                                    if pos.x != other_player.x || pos.y != other_player.y {
                                        //println!("Got a position :{:?}", pos);
                                    }
                                    other_player.x = pos.x;
                                    other_player.y = pos.y;
                                }
                            }
                            PlayerPacket::PlayerWelcomePacket(welc) => {
                                println!("Got a welcome packet");
                                // if self or already received return
                                let found = other_players.contains_key(&welc.player_id)
                                    || welc.player_id == player.id;
                                if !found {
                                    let mut temp = Player::new(welc.player_id);
                                    temp.x = welc.x;
                                    temp.y = welc.y;

                                    other_players.insert(temp.id, temp);
                                }
                            }
                            PlayerPacket::PlayerDisconnectPacket(disconnected) => {
                                println!("Got a disconnect packet");
                                other_players.remove(&disconnected.id);
                            }
                            PlayerPacket::PlayerAnimationPacket(animation) => {
                                println!("Got an animation packet");
                                if let Some(other_player) = other_players.get_mut(&animation.id) {
                                    other_player.animation_data =
                                        Some(animation.animation_data.clone());
                                    other_player
                                        .animation_data
                                        .as_mut()
                                        .unwrap()
                                        .load_animation(
                                            animation.animation_data.frames[0].path.clone(),
                                            0,
                                            0,
                                            16,
                                            16,
                                            3,
                                            &texture_creator,
                                            texture_map,
                                        );
                                }
                            }
                            PlayerPacket::PlayerLevelPacket(level) => {
                                println!("Got a level packet");
                                if let Some(other_player) = other_players.get_mut(&level.player_id)
                                {
                                    other_player.current_level = level.level.clone();
                                }
                            }
                        }
                    }
                    Packet::ClientIDPacket(id) => {
                        println!("Got an id :{}", id.id);
                        if player.id == 1_000_000 {
                            player.id = id.id;
                        }
                        let data = PlayerWelcome {
                            player_id: player.id,
                            x: player.x,
                            y: player.y,
                        };
                        self.packet_sender
                            .send(Packet::PlayerPacket(PlayerPacket::PlayerWelcomePacket(
                                data,
                            )))
                            .unwrap();
                        let data = PlayerAnimation {
                            id: player.id,
                            animation_data: player.animation_data.clone().unwrap(),
                        };
                        self.packet_sender
                            .send(Packet::PlayerPacket(PlayerPacket::PlayerAnimationPacket(
                                data,
                            )))
                            .unwrap();
                        self.packet_sender
                            .send(Packet::PlayerPacket(PlayerPacket::PlayerLevelPacket(
                                PlayerLevel {
                                    player_id: player.id,
                                    level: player.current_level.clone(),
                                },
                            )))
                            .unwrap();
                    }
                }
            }
            Err(mspc::TryRecvError::Empty) => (),
            Err(mspc::TryRecvError::Disconnected) => panic!("Disconnected"),
        }
    }

    // pause function
    pub fn pause(&mut self) {
        match self.game_state {
            GameState::Running => self.game_state = GameState::Paused,
            GameState::Paused => self.game_state = GameState::Running,
            GameState::GameOver => (),
        }
    }

    // main game loop
    pub fn run(&mut self) {
        let initial_level = "resources/levels/level1_1.png".to_string();

        // initalize sdl2 stuff
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("sea2d", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let mut canvas = window
            .into_canvas()
            .index(find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();

        // font rendering setup
        let ttf_context = ttf::init().unwrap();
        // Load a font
        let font_path = "resources/fonts/Battle-Race.ttf";
        let font = ttf_context.load_font(font_path, 56).unwrap();

        //sound
        //mixer::init(mixer::InitFlag::MP3 | mixer::InitFlag::OGG).unwrap();
        //mixer::open_audio(22050, mixer::DEFAULT_FORMAT, 2, 4096).unwrap();

        //let test_sound_effect = Chunk::from_file("resources/sound/effects/795424__koolkatbenziboii4__step-dirt-1.mp3").unwrap();
        //let test_music = Music::from_file("resources/sound/music/music.mp3").unwrap();

        //test_music.play(-1).unwrap();

        // --------------------------------------
        let viewport = rect::Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
        canvas.set_viewport(viewport);
        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut other_players: HashMap<u64, Player> = HashMap::new();

        // texture setup
        image::init(image::InitFlag::PNG | image::InitFlag::JPG).unwrap();
        let texture_creator = canvas.texture_creator();
        let mut texture_map: HashMap<String, Texture> = HashMap::new();

        // level loading
        let mut level = Level::new();
        level.load_from_file(initial_level.clone(), &texture_creator, &mut texture_map);

        // player setup
        let mut player = Player::new(1_000_000);
        player.animation_data = Some(AnimatedTexture::new(1.0 / 6.));
        player.animation_data.as_mut().unwrap().load_animation(
            "resources/player_animation/player.png".to_string(),
            0,
            0,
            16,
            16,
            3,
            &texture_creator,
            &mut texture_map,
        );
        player.animation_data.as_mut().unwrap().animation_type = AnimationType::PingPong;
        player.x = level.player_spawn.0 as f64;
        player.y = level.player_spawn.1 as f64;
        player.hitbox.x = player.x + 10.;
        player.hitbox.y = player.y + 15.;
        player.current_level = initial_level.clone();

        // camera

        let mut camera = Camera::new(
            player.x + (player.size as i32 / 2 - SCREEN_WIDTH as i32 / 2) as f64,
            player.y + (player.size as i32 / 2 - SCREEN_HEIGHT as i32 / 2) as f64,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        );

        // enemies

        let mut enemies: Vec<Enemy> = Vec::new();
        enemies.push(Enemy::new());
        enemies.last_mut().unwrap().animation_data = Some(AnimatedTexture::new(1.0 / 6.));
        enemies
            .last_mut()
            .unwrap()
            .animation_data
            .as_mut()
            .unwrap()
            .load_animation(
                "resources/textures/enemy.png".to_string(),
                0,
                0,
                16,
                16,
                3,
                &texture_creator,
                &mut texture_map,
            );
        enemies
            .last_mut()
            .unwrap()
            .animation_data
            .as_mut()
            .unwrap()
            .animation_type = AnimationType::PingPong;

        // hud
        let pause_button = Button::create_pause_button(self);
        let mut hud = Hud::new(vec![pause_button]);
        let mut draw_hitboxes = false;

        //testni gumb
        // fn fun() {
        //     println!("It's alive!")
        // }
        // let mut gumbek = Button::new(fun);

        let global_clock = std::time::Instant::now();
        let mut current_time = std::time::Instant::now();
        let time_step = 1.0 / 60.0;

        self.game_state = GameState::Running;

        'running: loop {
            // event polling
            for event in event_pump.poll_iter() {
                match self.game_state {
                    GameState::Running => player.on_event(&event),
                    GameState::Paused => player.reset_velocity(),
                    GameState::GameOver => (),
                }
                //camera.handle_zoom(&event);
                match event {
                    sdl2::event::Event::Quit { .. }
                    | sdl2::event::Event::KeyDown {
                        keycode: Some(sdl2::keyboard::Keycode::ESCAPE),
                        ..
                    } => break 'running,
                    sdl2::event::Event::KeyDown {
                        keycode: Some(sdl2::keyboard::Keycode::H),
                        ..
                    } => {
                        draw_hitboxes = !draw_hitboxes;
                    }
                    sdl2::event::Event::KeyDown {
                        keycode: Some(sdl2::keyboard::Keycode::P),
                        ..
                    } => match self.game_state {
                        GameState::Paused => self.game_state = GameState::Running,
                        GameState::Running => self.game_state = GameState::Paused,
                        GameState::GameOver => (),
                    },
                    sdl2::event::Event::KeyDown {
                        keycode: Some(sdl2::keyboard::Keycode::R),
                        ..
                    } => match self.game_state {
                        GameState::GameOver => {
                            self.game_state = GameState::Running;
                            player.health = 100;
                            player.pressed_down = false;
                            player.pressed_left = false;
                            player.pressed_right = false;
                            player.pressed_up = false;
                        }
                        _ => (),
                    },
                    sdl2::event::Event::KeyDown {
                        keycode: Some(sdl2::keyboard::Keycode::L),
                        ..
                    } => {
                        // sdl2::mixer::Channel::all().play(&test_sound_effect, 1).unwrap();
                    }
                    //klik na gumb
                    sdl2::event::Event::MouseButtonDown {
                        timestamp,
                        window_id,
                        which,
                        mouse_btn,
                        clicks,
                        x,
                        y,
                    } => {
                        for but in &mut hud.buttons {
                            but.handle_event(&event)
                        }
                    }

                    _ => {}
                }
            }

            // check if we need to load a new level
            if let Some(exit) = player.reached_end.clone() {
                level.load_from_file(exit.next_level.clone(), &texture_creator, &mut texture_map);
                player.x = level.player_spawn.0 as f64;
                player.y = level.player_spawn.1 as f64;
                player.hitbox.x = player.x + 10.0;
                player.hitbox.y = player.y + 15.0;
                camera.x = player.x + (player.size as i32 / 2 - SCREEN_WIDTH as i32 / 2) as f64;
                camera.y = player.y + (player.size as i32 / 2 - SCREEN_HEIGHT as i32 / 2) as f64;
                player.reached_end = None;
                player.current_level = exit.next_level.clone();
                self.packet_sender
                    .send(Packet::PlayerPacket(PlayerPacket::PlayerLevelPacket(
                        PlayerLevel {
                            player_id: player.id,
                            level: player.current_level.clone(),
                        },
                    )))
                    .unwrap();
            }

            // time handling
            let new_time = std::time::Instant::now();
            let mut frame_time = new_time - current_time;
            current_time = new_time;

            // update
            //  while frame_time > std::time::Duration::from_secs_f64(0.0){
            let delta_time = f64::min(frame_time.as_secs_f64(), time_step);

            match self.game_state {
                GameState::Running => {
                    for enemy in &mut enemies {
                        enemy.update(delta_time, &level, &player, &global_clock);
                    }
                    player.update(
                        delta_time,
                        &self.packet_sender,
                        &level,
                        &mut camera,
                        &enemies,
                        &global_clock,
                    );
                    for (_, other_player) in &mut other_players {
                        if !other_player.animation_data.is_none() {
                            other_player
                                .animation_data
                                .as_mut()
                                .unwrap()
                                .update(delta_time);
                        }
                    }
                }
                _ => (),
            }

            frame_time -= std::time::Duration::from_secs_f64(delta_time);
            std::thread::sleep(frame_time);
            // }

            if player.health <= 0 {
                match self.game_state {
                    GameState::GameOver => (),
                    _ => {
                        self.game_state = GameState::GameOver;
                        player.reset_velocity();
                    }
                }
            }

            // drawing
            canvas.set_blend_mode(sdl2::render::BlendMode::None);
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();

            //let viewport = rect::Rect::new(0,0,SCREEN_WIDTH,SCREEN_HEIGHT);
            let viewport = rect::Rect::new(0, 0, camera.width, camera.height);
            canvas.set_viewport(viewport);

            // draw level
            level.draw(&mut canvas, &texture_map, &camera);
            if draw_hitboxes {
                level.draw_hitboxes(&mut canvas, &camera);
            }
            // draw enemies
            for enemy in &enemies {
                enemy.draw(&mut canvas, &texture_map, &camera);
            }
            //draw other player if on the same level
            for (_, other_player) in &mut other_players {
                if other_player.current_level == player.current_level {
                    other_player.draw(&mut canvas, &texture_map, &camera, &global_clock);
                }
            }
            // draw self
            player.draw(&mut canvas, &texture_map, &camera, &global_clock);
            let player_hitbox_color = if player.colliding {
                Color::RED
            } else {
                Color::GREEN
            };

            if draw_hitboxes {
                player
                    .hitbox
                    .draw(&mut canvas, player_hitbox_color, &camera);
                for enemy in &enemies {
                    enemy.hitbox.draw(&mut canvas, Color::RED, &camera);
                }
            }

            //larine stvari
            hud.draw(&mut canvas);

            // clear screen
            match self.game_state {
                GameState::Paused | GameState::GameOver => {
                    canvas.set_draw_color(sdl2::pixels::Color::RGBA(00, 00, 255, 150));
                    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
                    canvas
                        .fill_rect(rect::Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT))
                        .unwrap();
                }
                _ => (),
            }
            match self.game_state {
                GameState::GameOver => {
                    let color = Color::RGB(255, 255, 255);
                    let surface = font.render("Game Joever").blended(color).unwrap(); // Create a blended surface (anti-aliased)

                    let texture = texture_creator
                        .create_texture_from_surface(&surface)
                        .unwrap();

                    // Get the size of the texture
                    let TextureQuery { width, height, .. } = texture.query();

                    // Set up the destination rectangle (where the text will appear)
                    let dest_rect = Rect::new(
                        (SCREEN_WIDTH / 2) as i32 - 200,
                        (SCREEN_HEIGHT / 2) as i32 - 30,
                        width,
                        height,
                    ); // Position at (100, 100)

                    // Clear the screen and draw the texture
                    canvas.copy(&texture, None, Some(dest_rect)).unwrap();
                }
                _ => (),
            }

            canvas.present();

            // send updates
            if player.id != 1_000_000 && player.moved {
                //println!("sendingk");
                self.packet_sender
                    .send(Packet::PlayerPacket(PlayerPacket::PlayerPositionPacket(
                        PlayerPosition {
                            player_id: player.id,
                            x: player.x,
                            y: player.y,
                        },
                    )))
                    .unwrap();
            }

            // receive
            self.handle_receive(
                &mut player,
                &mut other_players,
                &texture_creator,
                &mut texture_map,
            );
        }
    }
}
