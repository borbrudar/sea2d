use serde_derive::{Deserialize, Serialize};
use std::error::Error;

use std::any::TypeId;
use std::hash::{Hash, Hasher};

use fnv::FnvHasher;

use crate::player::PlayerPacket;


#[derive(Serialize,Deserialize,Debug,Clone)]

pub enum Packet{
    ClientIDPacket(ClientID),
    PlayerPacket(PlayerPacket),
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ClientID{
    pub id : u64,
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
        let check_type = get_type_id::<T>();
        if self.type_id != check_type {
            return None;
        }
        bincode::deserialize(&self.data).map_or_else(|_| None, |data| Some(data))
    }
}

