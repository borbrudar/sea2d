use serde::{Deserialize, Serialize};
use crate::shared::{SCREEN_HEIGHT,SCREEN_WIDTH};


pub struct Player{
    pub id : u64,
    pub x : i32,
    pub y : i32,
    pub color : (u8,u8,u8),
}

impl Player{
    pub fn new(id : u64) -> Player {
        Player{
            id : id,
            x : (SCREEN_WIDTH as i32)/2,
            y : (SCREEN_HEIGHT as i32)/2,
            color : (255,255,255)
        }
    }
    
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub enum PlayerPacket{
    PlayerIDPacket(PlayerID),
    PlayerMovementPacket(PlayerMovement),
    PlayerPositionPacket(PlayerPosition)
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
pub struct PlayerMovement{
    pub mov : Movement
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct PlayerID{
    pub id : u64
}
