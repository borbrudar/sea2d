use serde_derive::{Deserialize, Serialize};
use std::error::Error;

use std::any::TypeId;
use std::hash::{Hash, Hasher};

use fnv::FnvHasher;


#[derive(Serialize,Deserialize,Debug,Clone)]

pub enum Packet{
    PlayerMovementPacket(PlayerMovement),
}


#[derive(Serialize,Deserialize,Debug,Clone)]
pub enum Movement{
    Left,
    Right,
    Up,
    Down
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct PlayerMovement{
    pub mov : Movement
}



fn get_type_id<Type: 'static>() -> u64 {
    let mut hasher = FnvHasher::default();
    let type_id = TypeId::of::<Type>();
    type_id.hash(&mut hasher);
    hasher.finish()
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PacketInternal {
    pub type_id : u64,
    pub data: Vec<u8>,
}
impl PacketInternal{
    pub fn new<T: serde::Serialize + 'static>(data : T) -> Result<Self,Box<dyn Error>> {
        let type_id  = get_type_id::<T>();
        let data = bincode::serialize(&data)?;
        Ok(Self{type_id,data})
    }

    pub fn try_deserialize<T: serde::de::DeserializeOwned + 'static>(&self) -> Option<T> {
        if self.type_id != get_type_id::<T>() {
            return None;
        }
        bincode::deserialize(&self.data).map_or_else(|_| None, |data| Some(data))
    }
}

