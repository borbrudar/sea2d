use crate::aabb::AABB;
use crate::animated_texture::AnimatedTexture;
use crate::level::Level;
use crate::packet::Packet;
use crate::point::Point;
use crate::shared::{SCREEN_HEIGHT,SCREEN_WIDTH};
use crate::tile_type::ExitTile;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::render::Texture;
use crate::camera::Camera;
use crate::player_packets::{PlayerPacket, PlayerPosition};


pub struct Player{
    pub id : u64,
    pub x : f64,
    pub y : f64,
    velocity_x : f64,
    velocity_y : f64,
    pub size : u32,
    pub animation_data : Option<AnimatedTexture>,
    pub hitbox : AABB,
    pub colliding : bool,
    speed : f64,
    pub reached_end : Option<ExitTile>,

    pressed_up : bool,
    pressed_down : bool,
    pressed_left : bool,
    pressed_right : bool,

    pub current_level : String,
}

impl Player{
    pub fn new(id : u64) -> Player{
        Player{
            id : id,
            x : ((SCREEN_WIDTH as i32)/2) as f64,
            y : ((SCREEN_HEIGHT as i32)/2) as f64,
            velocity_x : 0.0,
            velocity_y : 0.0,
            size : 50,
            animation_data : None,
            hitbox : AABB::new(((SCREEN_WIDTH as i32)/2) as f64 + 10.0,((SCREEN_HEIGHT as i32)/2)as f64+15.0,30,30),
            colliding : false,
            speed : 250.0,
            reached_end : None,
            pressed_up : false,
            pressed_down : false,
            pressed_left : false,
            pressed_right : false,
            current_level : String::new()
        }
    }

    pub fn reset_velocity(&mut self) {
        self.velocity_x = 0.0;
        self.velocity_y = 0.0;
        self.pressed_down = false;
        self.pressed_left = false;
        self.pressed_right = false;
        self.pressed_up = false;
    }

    pub fn draw(&self,canvas : &mut Canvas<Window>, texture_map : &std::collections::HashMap<String,Texture>, camera : &Camera){ 
        match self.animation_data {
            Some(ref animation_data) => {
                //println!("Drawing animation");
                animation_data.draw(canvas,texture_map,self.x-camera.x,self.y-camera.y,self.size,self.size);
            },
            None => {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(255,192,203));
                canvas.fill_rect(sdl2::rect::Rect::new((self.x -camera.x) as i32,(self.y-camera.y)as i32,self.size,self.size)).unwrap();
            }
        }
    }

    pub fn update(&mut self, dt : f64,tx : &std::sync::mpsc::Sender<Packet>, level : &Level, camera : &mut Camera){
        match self.animation_data {
            Some(ref mut animation_data) => {
                animation_data.update(dt);
            },
            None => ()
        }

        if self.velocity_x == 0.0 && self.velocity_y == 0.0 {
            return;
        }
        

        if self.velocity_x != 0.0 && self.velocity_y != 0.0 {
            self.x += self.velocity_x * dt * 0.7071; // sqrt(2)/2
            self.y += self.velocity_y * dt * 0.7071;
            self.hitbox.x += self.velocity_x * dt * 0.7071;
            self.hitbox.y += self.velocity_y * dt * 0.7071; 
        }
        else{   
            self.x += self.velocity_x * dt;
            self.y += self.velocity_y * dt;
            self.hitbox.x += self.velocity_x * dt;
            self.hitbox.y += self.velocity_y * dt;
        }
                 
        let collisions   = level.check_collision(&self.hitbox);
        if collisions.len() > 0 {
            self.colliding = true;
        }else {
            self.colliding = false;
        }
        for tile in collisions {
            match tile._tile_type {
                crate::tile_type::TileType::Exit(inner) => {
                    self.reached_end = Some(inner.clone());
                }
                _ => ()
            }
        }

        level.resolve_collision(&mut self.hitbox);
        self.x = self.hitbox.x - 10.;
        self.y = self.hitbox.y - 15.;
        let send = Packet::PlayerPacket(PlayerPacket::PlayerPositionPacket(PlayerPosition{x : self.x, y : self.y, player_id: self.id}));
        tx.send(send).unwrap();


        camera.x = self.x + (self.size as i32/2 - SCREEN_WIDTH as i32/2) as f64;
        camera.y = self.y + (self.size as i32/2 - SCREEN_HEIGHT as i32/2) as f64;
    }

    pub fn on_event(&mut self, event : &sdl2::event::Event){
        match event {
            sdl2::event::Event::KeyDown { keycode: Some(keycode), .. } => {
                match *keycode {
                    sdl2::keyboard::Keycode::Up | sdl2::keyboard::Keycode::W =>    {
                        self.velocity_y = -self.speed;
                        self.pressed_up = true;
                    },
                    sdl2::keyboard::Keycode::Down | sdl2::keyboard::Keycode::S =>  {
                        self.velocity_y = self.speed;
                        self.pressed_down = true;
                    },
                    sdl2::keyboard::Keycode::Left | sdl2::keyboard::Keycode::A =>  {
                        self.velocity_x = -self.speed;
                        self.pressed_left = true;
                    },
                    sdl2::keyboard::Keycode::Right | sdl2::keyboard::Keycode::D => {
                        self.velocity_x = self.speed;
                        self.pressed_right = true;
                    }
                    _ => ()
                }
            },
            sdl2::event::Event::KeyUp { keycode : Some(keycode), .. } => {
                match *keycode {
                    sdl2::keyboard::Keycode::Up | sdl2::keyboard::Keycode::W =>    {
                        self.pressed_up = false;
                        if self.pressed_down {
                            self.velocity_y = self.speed;
                        }else {
                            self.velocity_y = 0.0;
                        }
                    },
                    sdl2::keyboard::Keycode::Down | sdl2::keyboard::Keycode::S =>  {
                        self.pressed_down = false;
                        if self.pressed_up {
                            self.velocity_y = -self.speed;
                        }else {
                            self.velocity_y = 0.0;
                        }
                    },
                    sdl2::keyboard::Keycode::Left | sdl2::keyboard::Keycode::A =>  {
                        self.pressed_left = false;
                        if self.pressed_right {
                            self.velocity_x = self.speed;
                        }else {
                            self.velocity_x = 0.0;
                        }
                    },
                    sdl2::keyboard::Keycode::Right | sdl2::keyboard::Keycode::D => {
                        self.pressed_right = false;
                        if self.pressed_left {
                            self.velocity_x = -self.speed;
                        }else {
                            self.velocity_x = 0.0;
                        }
                    },
                    _ => ()
                }
            }
            _ => ()
        }
    }
}