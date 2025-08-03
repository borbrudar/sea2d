use crate::display::button::{Badge, Button, ButtonAction, Dropdown, HealthBar};
use crate::display::{hud::Hud, mainmenu::MainMenu};
use crate::entities::enemy::EnemyType;
use crate::entities::projectile::Projectile;
use crate::entities::{camera::Camera, enemy::Enemy, player::Player};
use crate::environment::{level::Level, texture_data::TextureData};
use crate::networking::{packet::Packet, player_packets::*, shared::*};
use crate::wfc::overlap::wfc_level_generator;
use sdl2::image::{self};
use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureQuery;
use sdl2::ttf;
use std::collections::HashMap;
use std::sync::mpsc as mspc;

#[derive(Clone, Copy, Debug)]
pub enum GameState {
    Running,
    Paused,
    GameOver,
    MainMenu,
}

pub struct Game {
    packet_receiver: mspc::Receiver<Packet>,
    packet_sender: mspc::Sender<Packet>,
    game_state: GameState,
}

pub fn find_sdl_gl_driver() -> Option<u32> {
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
        _texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        _texture_map: &mut HashMap<String, Texture<'a>>,
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
                            PlayerPacket::PlayerAnimationPacket(_) => {
                                println!(
                                    "Got an animation packet, but animations are disabled for now"
                                );
                            }
                            //PlayerPacket::PlayerAnimationPacket(animation) => {
                            //    println!("Got an animation packet");
                            //    if let Some(other_player) = other_players.get_mut(&animation.id) {
                            //        other_player.animation_data =
                            //            Some(animation.animation_data.clone());
                            //        other_player
                            //            .animation_data
                            //            .as_mut()
                            //            .unwrap()
                            //            .load_animation(
                            //                animation.animation_data.frames[0].path.clone(),
                            //                0,
                            //                0,
                            //                16,
                            //                16,
                            //                3,
                            //                &texture_creator,
                            //                texture_map,
                            //            );
                            //    }
                            //}
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
                        //let data = PlayerAnimation {
                        //    id: player.id,
                        //    animation_data: player.animation_data.clone().unwrap(),
                        //};
                        //self.packet_sender
                        //    .send(Packet::PlayerPacket(PlayerPacket::PlayerAnimationPacket(
                        //        data,
                        //    )))
                        //    .unwrap();
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

    // main game loop
    pub fn run(&mut self) {
        //generate first level
        wfc_level_generator(None);
        let initial_level = "resources/levels/level1/level1_1.png".to_string();

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
        player.load_player_texture(&texture_creator, &mut texture_map);

        player.x = level.player_spawn.0 as f64;
        player.y = level.player_spawn.1 as f64;
        player.hitbox.x = player.x + 16.;
        player.hitbox.y = player.y + 40.;
        player.current_level = initial_level.clone();

        // camera
        let mut camera = Camera::new(
            player.x + (player.size_x as i32 / 2 - SCREEN_WIDTH as i32 / 2) as f64,
            player.y + (player.size_y as i32 / 2 - SCREEN_HEIGHT as i32 / 2) as f64,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        );

        // enemies
        let mut enemies: Vec<Enemy> = Vec::new();
        enemies.push(Enemy::new(
            EnemyType::Wizard,
            &texture_creator,
            &mut texture_map,
        ));

        let mut projectiles = Vec::new();

        // hud
        let pavza = Button::new(
            ButtonAction::ChangeGameState(GameState::Paused),
            None,
            Some(TextureData::new("resources/textures/pause.png".to_string())),
            Color::RGB(255, 0, 0),
            Rect::new(100, 0, 50, 50),
        );

        let resume = Button::new(
            ButtonAction::ChangeGameState(GameState::Running),
            None,
            Some(TextureData::new(
                "resources/textures/resume-2.png".to_string(),
            )),
            Color::RGB(0, 255, 0),
            Rect::new(50, 0, 50, 50),
        );
        //Health bar
        let healthbar = HealthBar::new();

        //Badges
        // let first_badge = Badge::new(
        //     Rect::new(300, 0, 50, 50),
        //     TextureData::new("resources/textures/scuba_mask.png".to_string()),
        // );

        //dropdown menu
        let ddm = Dropdown::new(
            Button::new(
                ButtonAction::Callback(Box::new(|| println!("Dropdown triggered"))),
                Some("...".to_string()),
                None,
                Color::RGB(0, 0, 0),
                Rect::new(0, 0, 50, 50),
            ),
            vec![
                Button::new(
                    ButtonAction::ChangeGameState(GameState::MainMenu),
                    Some("Back to Main Menu".to_string()),
                    None,
                    Color::RGB(214, 2, 112),
                    Rect::new(0, 50, 300, 50),
                ),
                Button::new(
                    ButtonAction::Callback(Box::new(|| println!("Item 2 clicked"))),
                    Some("Character descriptions".to_string()),
                    None,
                    Color::RGB(155, 79, 150),
                    Rect::new(0, 100, 300, 50),
                ),
                Button::new(
                    ButtonAction::Callback(Box::new(|| println!("Item 3 clicked"))),
                    Some("Item 3".to_string()),
                    None,
                    Color::RGB(0, 56, 168),
                    Rect::new(0, 150, 300, 50),
                ),
            ],
        );

        let global_clock = std::time::Instant::now();
        let mut current_time = std::time::Instant::now();
        let time_step = 1.0 / 60.0;
        let mut last_time_clicked = 0.0;

        let mut hud = Hud::new(
            vec![pavza, resume],
            Vec::new(),
            ddm,
            healthbar,
            current_time,
        );
        let mut draw_hitboxes = false;
        let mut draw_hud = true;

        self.game_state = GameState::Running;

        'running: loop {
            // event polling
            for event in event_pump.poll_iter() {
                match self.game_state {
                    GameState::Running => player.on_event(&event),
                    GameState::Paused => player.reset_velocity(),
                    GameState::GameOver => (),
                    GameState::MainMenu => (),
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
                        GameState::MainMenu => (),
                    },
                    sdl2::event::Event::KeyDown {
                        keycode: Some(sdl2::keyboard::Keycode::R),
                        ..
                    } => {
                        if let GameState::GameOver = self.game_state {
                            self.game_state = GameState::Running;
                            player.health = 100;
                            player.pressed_down = false;
                            player.pressed_left = false;
                            player.pressed_right = false;
                            player.pressed_up = false;
                        }
                    }
                    sdl2::event::Event::KeyDown {
                        keycode: Some(sdl2::keyboard::Keycode::T),
                        ..
                    } => {
                        draw_hud = !draw_hud;
                    }
                    sdl2::event::Event::KeyDown {
                        keycode: Some(sdl2::keyboard::Keycode::L),
                        ..
                    } => {
                        // sdl2::mixer::Channel::all().play(&test_sound_effect, 1).unwrap();
                    }
                    //klik z miško
                    sdl2::event::Event::MouseButtonDown {
                        timestamp: _,
                        window_id: _,
                        which: _,
                        mouse_btn,
                        clicks: _,
                        x,
                        y,
                    } => {
                        if mouse_btn == sdl2::mouse::MouseButton::Left {
                            // delay to prevent spam
                            if last_time_clicked + 0.25
                                < std::time::Instant::elapsed(&global_clock).as_secs_f32()
                            {
                                last_time_clicked =
                                    std::time::Instant::elapsed(&global_clock).as_secs_f32();

                                projectiles.push(Projectile::new(
                                    player.x + player.size_x as f64 / 2.0,
                                    player.y + player.size_y as f64 / 2.0,
                                    15,
                                    ((y - (SCREEN_HEIGHT / 2) as i32) as f64)
                                        .atan2((x - (SCREEN_WIDTH / 2) as i32) as f64),
                                    true,
                                ));
                                projectiles
                                    .last_mut()
                                    .unwrap()
                                    .load_projectile_texture(&texture_creator, &mut texture_map);
                            }
                        }

                        for but in &mut hud.buttons {
                            but.handle_event(&event, &mut self.game_state);
                        }
                        for item in &mut hud.dropdown.items {
                            item.handle_event(&event, &mut self.game_state);
                        }
                    }
                    sdl2::event::Event::MouseMotion { .. } => {
                        hud.dropdown.handle_event(&event);
                    }
                    _ => {}
                }
            }

            // check if we need to load a new level
            if let Some(exit) = player.reached_end.clone() {
                //generate new level
                wfc_level_generator(Some(player.current_level));
                //load level
                level.load_from_file(exit.next_level.clone(), &texture_creator, &mut texture_map);
                player.x = level.player_spawn.0 as f64;
                player.y = level.player_spawn.1 as f64;
                player.hitbox.x = player.x + 20.0;
                player.hitbox.y = player.y + 76.0;
                camera.x = player.x + (player.size_x as i32 / 2 - SCREEN_WIDTH as i32 / 2) as f64;
                camera.y = player.y + (player.size_y as i32 / 2 - SCREEN_HEIGHT as i32 / 2) as f64;
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

            if let GameState::Running = self.game_state {
                for enemy in &mut enemies {
                    let prev_size = projectiles.len();
                    enemy.update(delta_time, &level, &player, &global_clock, &mut projectiles);
                    if projectiles.len() > prev_size {
                        // if new projectiles were added, we need to load their textures
                        projectiles
                            .last_mut()
                            .unwrap()
                            .load_projectile_texture(&texture_creator, &mut texture_map);
                    }
                }
                player.update(
                    delta_time,
                    &self.packet_sender,
                    &level,
                    &mut camera,
                    &enemies,
                    &global_clock,
                );
                for other_player in other_players.values_mut() {
                    other_player.animation_data.update(delta_time);
                }
                // update projectiles
                let mut remove_projectiles = Vec::new();
                for projectile in &mut projectiles {
                    projectile.update(delta_time);
                    if projectile.resolve_collision(&level, &mut enemies, &mut player) {
                        // remove projectile if it collides with something
                        remove_projectiles.push(projectile.clone());
                    }
                }
                for projectile in &remove_projectiles {
                    if let Some(pos) = projectiles
                        .iter()
                        .position(|p| p.x == projectile.x && p.y == projectile.y)
                    {
                        projectiles.remove(pos);
                    }
                }
                // remove dead enemies
                enemies.retain(|enemy| enemy.health > 0);
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

            // draw projectiles
            for projectile in &mut projectiles {
                projectile.draw(&mut canvas, &texture_map, &camera);
            }

            //draw other player if on the same level
            for other_player in other_players.values_mut() {
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
                for projectile in &projectiles {
                    projectile.hitbox.draw(&mut canvas, Color::RED, &camera);
                }
            }

            //hud
            if draw_hud {
                hud.draw(
                    player.health,
                    &mut canvas,
                    &ttf_context,
                    &texture_creator,
                    &mut texture_map,
                );
            }

            // clear screen
            match self.game_state {
                GameState::Paused | GameState::GameOver => {
                    canvas.set_draw_color(sdl2::pixels::Color::RGBA(00, 00, 255, 150));
                    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
                    canvas
                        .fill_rect(rect::Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT))
                        .unwrap();
                }
                GameState::MainMenu => {
                    let mut main_menu = MainMenu::new("Game needs a new title".to_string());
                    main_menu.draw(
                        &mut canvas,
                        &ttf_context,
                        &texture_creator,
                        &mut texture_map,
                    );
                    for event in event_pump.poll_iter() {
                        main_menu
                            .start_button
                            .handle_event(&event, &mut self.game_state);
                        match event {
                            sdl2::event::Event::Quit { .. }
                            | sdl2::event::Event::KeyDown {
                                keycode: Some(sdl2::keyboard::Keycode::ESCAPE),
                                ..
                            } => break 'running,
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
            if let GameState::GameOver = self.game_state {
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
            };

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
