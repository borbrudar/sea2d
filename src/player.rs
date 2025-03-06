use crate::aabb::AABB;
use crate::animated_texture::AnimatedTexture;
use crate::level::Level;
use crate::packet::Packet;
use crate::shared::{SCREEN_HEIGHT,SCREEN_WIDTH};
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::texture_data::TextureData;
use sdl2::render::Texture;
use crate::camera::Camera;
use crate::player_packets::{PlayerPacket,PlayerMovement,Movement};


pub struct Player{
    pub id : u64,
    pub x : i32,
    pub y : i32,
    pub color : (u8,u8,u8),
    size : u32,
    pub texture_data : Option<TextureData>,
    pub animation_data : Option<AnimatedTexture>,
    pub hitbox : AABB,
    pub colliding : bool,
    speed : i32,
}

impl Player{
    pub fn new(id : u64) -> Player{
        Player{
            id : id,
            x : (SCREEN_WIDTH as i32)/2,
            y : (SCREEN_HEIGHT as i32)/2,
            color : (255,255,255),
            size : 120,
            texture_data : None,
            animation_data : None,
            hitbox : AABB::new((SCREEN_WIDTH as i32)/2+40,(SCREEN_HEIGHT as i32)/2+80,40,40),
            colliding : false,
            speed : 15,
        }
    }

    pub fn draw(&self,canvas : &mut Canvas<Window>, texture_map : &std::collections::HashMap<String,Texture>, camera : &Camera){ 
        match self.animation_data {
            Some(ref animation_data) => {
                //println!("Drawing animation");
                animation_data.draw(canvas,texture_map,self.x-camera.x,self.y-camera.y,self.size,self.size);
            },
            None => match self.texture_data {
                Some (ref texture_data) => {
                    let res = texture_data.draw(canvas,texture_map,self.x  -camera.x,self.y-camera.y,self.size,self.size);
                    match res {
                        Err(..) => {
                            canvas.set_draw_color(sdl2::pixels::Color::RGB(self.color.0,self.color.1,self.color.2));
                            canvas.fill_rect(sdl2::rect::Rect::new(self.x-camera.x,self.y-camera.y,self.size,self.size)).unwrap();
                        },
                        Ok(..) => ()
                    }
                },
                None => {
                    canvas.set_draw_color(sdl2::pixels::Color::RGB(255,192,203));
                    canvas.fill_rect(sdl2::rect::Rect::new(self.x -camera.x,self.y-camera.y,self.size,self.size)).unwrap();
                }
            }
        }
    }

    pub fn on_event(&mut self, event : sdl2::event::Event, tx : &std::sync::mpsc::Sender<Packet>, level : &Level, camera : &mut Camera){
        match event {
            sdl2::event::Event::KeyDown { keycode: Some(keycode), .. } => {
                match keycode {
                    sdl2::keyboard::Keycode::Up => {
                        camera.y -= self.speed;
                        self.y -= self.speed;
                        let send = Packet::PlayerPacket(PlayerPacket::PlayerMovementPacket(PlayerMovement{mov : Movement::Down}));
                        tx.send(send).unwrap();
                    },
                    sdl2::keyboard::Keycode::Down => {
                        camera.y += self.speed;
                        self.y += self.speed;
                        let send = Packet::PlayerPacket(PlayerPacket::PlayerMovementPacket(PlayerMovement{mov : Movement::Up}));
                        tx.send(send).unwrap();
                    },
                    sdl2::keyboard::Keycode::Left => {
                        camera.x -= self.speed;
                        self.x -= self.speed;
                        let send = Packet::PlayerPacket(PlayerPacket::PlayerMovementPacket(PlayerMovement{mov : Movement::Left}));
                        tx.send(send).unwrap();
                    },
                    sdl2::keyboard::Keycode::Right => {
                        camera.x += self.speed;
                        self.x += self.speed;
                        let send = Packet::PlayerPacket(PlayerPacket::PlayerMovementPacket(PlayerMovement{mov : Movement::Right}));
                        tx.send(send).unwrap();
                    },
                    _ => ()
                }
            },
            _ => ()
        }
        if self.check_collision(level) {
            self.colliding = true;
        }
    }
    fn check_collision(&self, level : &Level) -> bool{
        for layer in &level.tiles{
            for tile in layer{
                match tile.bounding_box{
                    Some(ref bounding_box) => {
                        if self.hitbox.intersects(bounding_box){
                            return true;
                        }
                    },
                    None => ()
                }
            }
        }
        false
    }
}