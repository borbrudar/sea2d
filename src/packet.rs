use fnv::FnvHasher;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;

use std::any::TypeId;
use crate::player_packets::PlayerPacket;

use std::hash::{Hash,Hasher};


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
    //let type_id = TypeId::of::<Type>();
    let type_name = std::any::type_name::<Type>();
    //println!("Type id: {:?}",type_id);
    type_name.hash(&mut hasher);
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
    #[must_use]
    pub fn try_deserialize<T: serde::de::DeserializeOwned + 'static>(&self) -> Option<T> {
        if self.type_id == get_type_id::<T>() {
            println!("Type match");
            bincode::deserialize(&self.data).map_or_else(|_| None, |data| Some(data))
        } else {
            println!("Type mismatch");
            None
        }
    }
}
