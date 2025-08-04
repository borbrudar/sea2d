use crate::entities::animation_data::{AnimationData, AnimationState};
use crate::entities::projectile::{self, Projectile};
use crate::entities::{animated_texture::AnimatedTexture, camera::Camera, enemy::Enemy};
use crate::environment::{aabb::AABB, level::Level, tile_type::ExitTile};
use crate::networking::packet::Packet;
use crate::networking::shared::{SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::render::Texture;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::time::Instant;

pub enum PlayerHitState {
    Invincible,
    Vulnerable,
}

pub struct Player {
    pub id: u64,
    pub x: f64,
    pub y: f64,
    velocity_x: f64,
    velocity_y: f64,
    pub size_x: u32,
    pub size_y: u32,
    pub animation_data: AnimationData,
    pub hitbox: AABB,
    pub colliding: bool,
    speed: f64,
    pub reached_end: Option<ExitTile>,

    pub pressed_up: bool,
    pub pressed_down: bool,
    pub pressed_left: bool,
    pub pressed_right: bool,

    pub current_level: String,
    pub hit_state: PlayerHitState,
    pub health: i32,
    last_hit_time: f64,
    last_heal_time: f64,
    last_moved_time: f64,
    invicibility_blinks: i32,
    last_blink_time: f64,
    pub moved: bool,
}

impl Player {
    pub fn new(id: u64) -> Player {
        Player {
            id,
            x: ((SCREEN_WIDTH as i32) / 2) as f64,
            y: ((SCREEN_HEIGHT as i32) / 2) as f64,
            velocity_x: 0.0,
            velocity_y: 0.0,
            size_x: 36 * 2,
            size_y: 48 * 2,
            animation_data: AnimationData::new(),

            hitbox: AABB::new(
                ((SCREEN_WIDTH as i32) / 2) as f64,
                ((SCREEN_HEIGHT as i32) / 2) as f64 + 76.0,
                36,
                20,
            ),
            colliding: false,
            speed: 250.0,
            reached_end: None,
            pressed_up: false,
            pressed_down: false,
            pressed_left: false,
            pressed_right: false,
            current_level: String::new(),
            hit_state: PlayerHitState::Vulnerable,
            health: 100,
            last_hit_time: 0.0,
            last_heal_time: 0.0,
            invicibility_blinks: 0,
            last_blink_time: 0.0,
            moved: false,
            last_moved_time: 0.0,
        }
    }

    pub fn load_player_texture<'a>(
        &mut self,
        texture_creator: &'a TextureCreator<WindowContext>,
        texture_map: &mut std::collections::HashMap<String, Texture<'a>>,
    ) {
        self.animation_data.front = Some(AnimatedTexture::new(1.0 / 12.));
        self.animation_data.front.as_mut().unwrap().load_animation(
            "resources/player_animation/pretnar_spritesheet.png".to_string(),
            0,
            0,
            32,
            48,
            6,
            texture_creator,
            texture_map,
        );
        self.animation_data.right = Some(AnimatedTexture::new(1.0 / 12.));
        self.animation_data.right.as_mut().unwrap().load_animation(
            "resources/player_animation/pretnar_spritesheet.png".to_string(),
            0,
            48,
            32,
            48,
            6,
            texture_creator,
            texture_map,
        );
        self.animation_data.left = Some(AnimatedTexture::new(1.0 / 12.));
        self.animation_data.left.as_mut().unwrap().load_animation(
            "resources/player_animation/pretnar_spritesheet.png".to_string(),
            0,
            96,
            32,
            48,
            6,
            texture_creator,
            texture_map,
        );
        self.animation_data.back = Some(AnimatedTexture::new(1.0 / 12.));
        self.animation_data.back.as_mut().unwrap().load_animation(
            "resources/player_animation/pretnar_spritesheet.png".to_string(),
            0,
            144,
            32,
            48,
            6,
            texture_creator,
            texture_map,
        );
        self.animation_data.default = Some(AnimatedTexture::new(1.0));
        self.animation_data
            .default
            .as_mut()
            .unwrap()
            .load_animation(
                "resources/player_animation/pretnar_spritesheet.png".to_string(),
                0,
                0,
                32,
                48,
                1,
                texture_creator,
                texture_map,
            );
        self.animation_data.idle = Some(AnimatedTexture::new(1.0 / 3.0));
        self.animation_data.idle.as_mut().unwrap().load_animation(
            "resources/player_animation/pretnar_spritesheet.png".to_string(),
            0,
            192,
            32,
            48,
            6,
            texture_creator,
            texture_map,
        );
    }

    pub fn reset_velocity(&mut self) {
        self.velocity_x = 0.0;
        self.velocity_y = 0.0;
        self.pressed_down = false;
        self.pressed_left = false;
        self.pressed_right = false;
        self.pressed_up = false;
    }

    pub fn draw(
        &mut self,
        canvas: &mut Canvas<Window>,
        texture_map: &std::collections::HashMap<String, Texture>,
        camera: &Camera,
        global_clock: &Instant,
    ) {
        match self.hit_state {
            PlayerHitState::Invincible => {
                let time_since_last_blink =
                    global_clock.elapsed().as_secs_f64() - self.last_blink_time;
                if time_since_last_blink < 0.1 {
                    return;
                }
                let mut draw = false;
                let time_since_hit = global_clock.elapsed().as_secs_f64() - self.last_hit_time;
                for i in 0..4 {
                    if self.invicibility_blinks <= i && time_since_hit > (i as f64) / 4. {
                        self.invicibility_blinks += 1;
                        self.last_blink_time = global_clock.elapsed().as_secs_f64();
                        draw = true;
                    }
                }
                if !draw {
                    self.animation_data.draw(
                        canvas,
                        texture_map,
                        self.x - camera.x,
                        self.y - camera.y,
                        self.size_x,
                        self.size_y,
                    );
                }
            }
            PlayerHitState::Vulnerable => {
                self.animation_data.draw(
                    canvas,
                    texture_map,
                    self.x - camera.x,
                    self.y - camera.y,
                    self.size_x,
                    self.size_y,
                );
            }
        }
    }

    pub fn update(
        &mut self,
        dt: f64,
        _tx: &std::sync::mpsc::Sender<Packet>,
        level: &Level,
        camera: &mut Camera,
        enemies: &mut Vec<Enemy>,
        projectiles: &Vec<Projectile>,
        global_clock: &Instant,
    ) {
        if self.id == 1_000_000 {
            return;
        }

        self.moved = false;
        if self.velocity_x != 0.0 && self.velocity_y != 0.0 {
            self.x += self.velocity_x * dt * 0.7071; // sqrt(2)/2
            self.y += self.velocity_y * dt * 0.7071;
            self.hitbox.x += self.velocity_x * dt * 0.7071;
            self.hitbox.y += self.velocity_y * dt * 0.7071;
            self.moved = true;
        } else {
            self.x += self.velocity_x * dt;
            self.y += self.velocity_y * dt;
            self.hitbox.x += self.velocity_x * dt;
            self.hitbox.y += self.velocity_y * dt;
            self.moved = true;
        }
        if self.velocity_x == 0.0 && self.velocity_y == 0.0 {
            self.moved = false;
        }

        if self.moved {
            if self.velocity_x > 0.0 {
                self.animation_data.current_animation = AnimationState::Right;
            } else if self.velocity_x < 0.0 {
                self.animation_data.current_animation = AnimationState::Left;
            } else if self.velocity_y > 0.0 {
                self.animation_data.current_animation = AnimationState::Front;
            } else if self.velocity_y < 0.0 {
                self.animation_data.current_animation = AnimationState::Back;
            }
            self.animation_data.update(dt);
            self.last_moved_time = global_clock.elapsed().as_secs_f64();
        } else if self.last_moved_time + 5.0 < global_clock.elapsed().as_secs_f64() {
            self.animation_data.update(dt);
            self.animation_data.current_animation = AnimationState::Idle;
        }

        let collisions = level.check_collision(&self.hitbox);
        self.colliding = !collisions.is_empty();
        for tile in collisions {
            if let crate::environment::tile_type::TileType::Exit(inner) = tile.tile_type {
                if inner.locked {
                    self.reached_end = None
                } else {
                    self.reached_end = Some(inner.clone());
                }
            }
        }

        for projectile in projectiles {
            if projectile.resolve_collision(level, enemies, self) && !projectile.fired_by_player {
                self.last_hit_time = global_clock.elapsed().as_secs_f64();
            }
        }

        for enemy in enemies {
            if self.hitbox.intersects(&enemy.hitbox) {
                if let PlayerHitState::Vulnerable = self.hit_state {
                    self.hit_state = PlayerHitState::Invincible;
                    self.health -= 15;
                    println!("Health : {}", self.health);
                    self.last_hit_time = global_clock.elapsed().as_secs_f64();
                }
            }
        }

        let difference = global_clock.elapsed().as_secs_f64() - self.last_hit_time;

        if difference > 1.0 {
            self.hit_state = PlayerHitState::Vulnerable;
            self.invicibility_blinks = 0;
        }

        //healing logic

        let now = global_clock.elapsed().as_secs_f64();
        let time_since_hit = now - self.last_hit_time;
        let time_since_heal = now - self.last_heal_time;

        if self.health < 100 && time_since_hit >= 3.0 && time_since_heal >= 3.0 {
            self.health = (self.health + 10).min(100);
            self.last_heal_time = now;
            println!("Healed! Current health: {}", self.health);
        }

        level.resolve_collision(&mut self.hitbox);
        self.x = self.hitbox.x - 20.;
        self.y = self.hitbox.y - 76.;
        //let send = Packet::PlayerPacket(PlayerPacket::PlayerPositionPacket(PlayerPosition{x : self.x, y : self.y, player_id: self.id}));
        //tx.send(send).unwrap();

        camera.x = self.x + (self.size_x as i32 / 2 - SCREEN_WIDTH as i32 / 2) as f64;
        camera.y = self.y + (self.size_y as i32 / 2 - SCREEN_HEIGHT as i32 / 2) as f64;
    }

    pub fn on_event(&mut self, event: &sdl2::event::Event) {
        match event {
            sdl2::event::Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match *keycode {
                sdl2::keyboard::Keycode::Up | sdl2::keyboard::Keycode::W => {
                    self.velocity_y = -self.speed;
                    self.pressed_up = true;
                }
                sdl2::keyboard::Keycode::Down | sdl2::keyboard::Keycode::S => {
                    self.velocity_y = self.speed;
                    self.pressed_down = true;
                }
                sdl2::keyboard::Keycode::Left | sdl2::keyboard::Keycode::A => {
                    self.velocity_x = -self.speed;
                    self.pressed_left = true;
                }
                sdl2::keyboard::Keycode::Right | sdl2::keyboard::Keycode::D => {
                    self.velocity_x = self.speed;
                    self.pressed_right = true;
                }
                _ => (),
            },
            sdl2::event::Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => match *keycode {
                sdl2::keyboard::Keycode::Up | sdl2::keyboard::Keycode::W => {
                    self.pressed_up = false;
                    if self.pressed_down {
                        self.velocity_y = self.speed;
                    } else {
                        self.velocity_y = 0.0;
                    }
                }
                sdl2::keyboard::Keycode::Down | sdl2::keyboard::Keycode::S => {
                    self.pressed_down = false;
                    if self.pressed_up {
                        self.velocity_y = -self.speed;
                    } else {
                        self.velocity_y = 0.0;
                    }
                }
                sdl2::keyboard::Keycode::Left | sdl2::keyboard::Keycode::A => {
                    self.pressed_left = false;
                    if self.pressed_right {
                        self.velocity_x = self.speed;
                    } else {
                        self.velocity_x = 0.0;
                    }
                }
                sdl2::keyboard::Keycode::Right | sdl2::keyboard::Keycode::D => {
                    self.pressed_right = false;
                    if self.pressed_left {
                        self.velocity_x = -self.speed;
                    } else {
                        self.velocity_x = 0.0;
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }
}
