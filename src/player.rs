use serde::{Deserialize, Serialize};



pub struct Player{
    pub id : u64,
    pub x : i32,
    pub y : i32,
    pub color : (u8,u8,u8),
}

impl Player{
    
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
