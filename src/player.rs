use serde::{Deserialize, Serialize};
use crate::shared::{SCREEN_HEIGHT,SCREEN_WIDTH};
use sdl2::render::Canvas;
use sdl2::video::Window;


pub struct Player{
    pub id : u64,
    pub x : i32,
    pub y : i32,
    pub color : (u8,u8,u8),
    size : u32,
}

impl Player{
    pub fn new(id : u64) -> Player {
        Player{
            id : id,
            x : (SCREEN_WIDTH as i32)/2,
            y : (SCREEN_HEIGHT as i32)/2,
            color : (255,255,255),
            size : 40,
        }
    }
    pub fn draw(&self,canvas : &mut Canvas<Window>){
        canvas.set_draw_color(sdl2::pixels::Color::RGB(self.color.0,self.color.1,self.color.2));
        canvas.fill_rect(sdl2::rect::Rect::new(self.x,self.y,self.size,self.size)).unwrap();
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
