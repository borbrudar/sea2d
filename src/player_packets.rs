use serde::{Deserialize,Serialize};

use crate::texture_data::TextureData;

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