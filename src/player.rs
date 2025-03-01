use sdl2::image::{self, LoadTexture};
use sdl2::libc::pid_t;
use serde::{Deserialize, Serialize};
use crate::shared::{SCREEN_HEIGHT,SCREEN_WIDTH};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::Window;
use crate::texture_data::TextureData;
use sdl2::render::Texture;

pub struct Player{
    pub id : u64,
    pub x : i32,
    pub y : i32,
    pub color : (u8,u8,u8),
    size : u32,
    pub texture_data : Option<TextureData>,
}

impl Player{
    pub fn new(id : u64) -> Player{
        Player{
            id : id,
            x : (SCREEN_WIDTH as i32)/2,
            y : (SCREEN_HEIGHT as i32)/2,
            color : (255,255,255),
            size : 40,
            texture_data : None
        }
    }

    pub fn draw(&self,canvas : &mut Canvas<Window>, texture_map : &std::collections::HashMap<TextureData,Texture>) {
        match self.texture_data {
            Some (ref texture_data) => {
                let res = texture_data.draw(canvas,texture_map,self.x,self.y,self.size,self.size);
                match res {
                    Err(..) => {
                        canvas.set_draw_color(sdl2::pixels::Color::RGB(self.color.0,self.color.1,self.color.2));
                        canvas.fill_rect(sdl2::rect::Rect::new(self.x,self.y,self.size,self.size)).unwrap();
                    },
                    Ok(..) => ()
                }
            },
            None => {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(255,192,203));
                canvas.fill_rect(sdl2::rect::Rect::new(self.x,self.y,self.size,self.size)).unwrap();
            }
        }
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub enum PlayerPacket{
    PlayerWelcomePacket(PlayerWelcome),
    PlayerMovementPacket(PlayerMovement),
    PlayerPositionPacket(PlayerPosition),
    PlayerTextureDataPacket(PlayerTextureData),
    PlayerDisconnectPacket(PlayerDisconnect),
}


#[derive(Serialize,Deserialize,Debug,Clone)]
pub enum Movement{
    Left,
    Right,
    Up,
    Down
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct PlayerPosition{
    pub player_id : u64,
    pub x : i32,
    pub y : i32,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct PlayerWelcome {
    pub player_id : u64,
    pub x : i32,
    pub y : i32,
    pub texture_data : Option<TextureData>,
}


#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct PlayerMovement{
    pub mov : Movement
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct PlayerTextureData{
    pub texture_data : TextureData,
    pub id : u64,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct PlayerDisconnect{
    pub id : u64
}