use crate::aabb::AABB;
use crate::animated_texture::AnimatedTexture;
use crate::level::Level;
use crate::packet::Packet;
use crate::shared::{SCREEN_HEIGHT,SCREEN_WIDTH};
use crate::tile::Tile;
use crate::tile_type::ExitTile;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::texture_data::TextureData;
use sdl2::render::Texture;
use crate::camera::Camera;
use crate::player_packets::{PlayerPacket, PlayerPosition};


pub struct Player{
    pub id : u64,
    pub x : i32,
    pub y : i32,
    pub color : (u8,u8,u8),
    pub size : u32,
    pub texture_data : Option<TextureData>,
    pub animation_data : Option<AnimatedTexture>,
    pub hitbox : AABB,
    pub colliding : bool,
    speed : i32,
    pub reached_end : Option<ExitTile>,
}

impl Player{
    pub fn new(id : u64) -> Player{
        Player{
            id : id,
            x : (SCREEN_WIDTH as i32)/2,
            y : (SCREEN_HEIGHT as i32)/2,
            color : (255,255,255),
            size : 50,
            texture_data : None,
            animation_data : None,
            hitbox : AABB::new((SCREEN_WIDTH as i32)/2 + 10,(SCREEN_HEIGHT as i32)/2+15,30,30),
            colliding : false,
            speed : 15,
            reached_end : None,
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

    pub fn on_event(&mut self, event : &sdl2::event::Event, tx : &std::sync::mpsc::Sender<Packet>, level : &Level, camera : &mut Camera){
        let mut updated = false;
        match event {
            sdl2::event::Event::KeyDown { keycode: Some(keycode), .. } => {
                match *keycode {
                    sdl2::keyboard::Keycode::Up => {
                        self.y -= self.speed;
                        self.hitbox.y -= self.speed;
                        updated = true;
                    },
                    sdl2::keyboard::Keycode::Down => {
                        self.y += self.speed;
                        self.hitbox.y += self.speed;
                        updated = true;
                    },
                    sdl2::keyboard::Keycode::Left => {
                        self.x -= self.speed;
                        self.hitbox.x -= self.speed;
                        updated = true;
                    },
                    sdl2::keyboard::Keycode::Right => {
                        self.x += self.speed;
                        self.hitbox.x += self.speed;
                        updated = true;
                    },
                    _ => ()
                }
            },
            _ => ()
        }
        if !updated {
            return;
        }
        self.resolve_collision(level);
        let send = Packet::PlayerPacket(PlayerPacket::PlayerPositionPacket(PlayerPosition{x : self.x, y : self.y, player_id: self.id}));
        tx.send(send).unwrap();

        camera.x = self.x + self.size as i32/2 - SCREEN_WIDTH as i32/2;
        camera.y = self.y + self.size as i32/2 - SCREEN_HEIGHT as i32/2;
        if self.check_collision(level) {
            self.colliding = true;
        }else {
            self.colliding = false;
        }
    }

    fn check_collision(&self, level : &Level) -> bool {
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

    fn resolve_collision(&mut self, level : &Level) {
        for layer in &level.tiles{
            for tile in layer{
                match tile.bounding_box{
                    Some(ref bounding_box) => {
                        if self.hitbox.intersects(bounding_box){
                            let x1 = self.hitbox.x + self.hitbox.w as i32 - bounding_box.x; // right side of player - left side of tile
                            let x2 = bounding_box.x + bounding_box.w as i32 - self.hitbox.x; // right side of tile - left side of player
                            let y1 = self.hitbox.y + self.hitbox.h as i32 - bounding_box.y; // bottom side of player - top side of tile
                            let y2 = bounding_box.y + bounding_box.h as i32 - self.hitbox.y; // bottom side of tile - top side of player
                            let min = x1.min(x2).min(y1).min(y2);
                            if min == x1 {
                                self.x -= x1;
                                self.hitbox.x -= x1;
                            }else if min == x2 {
                                self.x += x2;
                                self.hitbox.x += x2;
                            }else if min == y1 {
                                self.y -= y1;
                                self.hitbox.y -= y1;
                            }else if min == y2 {
                                self.y += y2;
                                self.hitbox.y += y2;
                            }
                            match &tile._tile_type {
                                crate::tile_type::TileType::Exit(inner) => {
                                    self.reached_end = Some((*inner).clone());
                                }
                                _ => ()
                            }
                        }
                    },
                    None => ()
                }
            }
        }
    }
}