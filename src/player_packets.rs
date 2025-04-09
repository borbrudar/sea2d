use serde::{Deserialize,Serialize};

use crate::animated_texture::AnimatedTexture;

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub enum PlayerPacket{
    PlayerWelcomePacket(PlayerWelcome),
    PlayerPositionPacket(PlayerPosition),
    PlayerDisconnectPacket(PlayerDisconnect),
    PlayerAnimationPacket(PlayerAnimation),
    PlayerLevelPacket(PlayerLevel),
}


#[derive(Serialize,Deserialize,Debug,Clone)]
pub enum Movement{
    Left,
    Right,
    Up,
    Down
}

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub struct PlayerPosition{
    pub player_id : u64,
    pub x : f64,
    pub y : f64,
}

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub struct PlayerWelcome {
    pub player_id : u64,
    pub x : f64,
    pub y : f64,
}

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub struct PlayerDisconnect{
    pub id : u64
}

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub struct PlayerAnimation{
    pub id : u64,
    pub animation_data : AnimatedTexture
}

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub struct PlayerLevel{
    pub player_id : u64,
    pub level : String,
}