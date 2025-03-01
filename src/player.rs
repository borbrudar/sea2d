use sdl2::image::{self, LoadTexture};
use sdl2::libc::pid_t;
use serde::{Deserialize, Serialize};
use crate::shared::{SCREEN_HEIGHT,SCREEN_WIDTH};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::Window;
use crate::thread_safe_texture::ThreadSafeTexture;

pub struct Player<'a>{
    pub id : u64,
    pub x : i32,
    pub y : i32,
    pub color : (u8,u8,u8),
    size : u32,
    pub texture : ThreadSafeTexture<'a>,
}

impl<'a> Player<'a>{
    pub fn new(id : u64) -> Player<'a>{
        Player{
            id : id,
            x : (SCREEN_WIDTH as i32)/2,
            y : (SCREEN_HEIGHT as i32)/2,
            color : (255,255,255),
            size : 40,
            texture : ThreadSafeTexture::new(),
        }
    }
    pub fn draw(&self,canvas : &mut Canvas<Window>){
        let res = self.texture.render(canvas, self.x, self.y, self.size, self.size);
        match res {
            Err(..) => {
                canvas.fill_rect(sdl2::rect::Rect::new(self.x,self.y,self.size,self.size)).unwrap();
                canvas.set_draw_color(sdl2::pixels::Color::RGB(self.color.0,self.color.1,self.color.2));
            },
            Ok(..) => ()
        }
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub enum PlayerPacket{
    PlayerWelcomePacket(PlayerWelcome),
    PlayerIDPacket(PlayerID),
    PlayerMovementPacket(PlayerMovement),
    PlayerPositionPacket(PlayerPosition),
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
}


#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct PlayerMovement{
    pub mov : Movement
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct PlayerID{
    pub id : u64
}
