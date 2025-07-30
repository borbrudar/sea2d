use std::collections::HashMap;

use crate::environment::{texture_data::TextureData, tile_type::TileType};

const TILE_SIZE : u32 = 16;

#[derive(Hash)]
pub enum TileSetType{
    Simple, // 1 tile
    Complex, // 3x3 
    Full, // 47
    FullVariants, // se variacije
}


pub struct Autotiler{
    pub tiles_info : HashMap<TileType, (TileSetType, String)>,
}

impl Autotiler {
    pub fn new() -> Autotiler {
        Autotiler {
            tiles_info: HashMap::new(),
        }
    }

    pub fn add_tile(&mut self, tile_type: TileType, tile_set_type: TileSetType, path: String) {
        self.tiles_info.insert(tile_type, (tile_set_type, path));
    }

    pub fn get_tile_texture(&self, neighbours: [[bool;3];3], tile_type: TileType) -> Option<TextureData> {
        // helper to check neighbours
        let check = |first : [i32;9], second : [[bool;3];3]| {
            let mut r = true;
            for i in 0..9 {
                if first[i] == 2 || i == 4 {continue;}
                if !((first[i] == 1) ^ !second[i/3][i%3]) {
                    r = false;
                    break;
                }
            }
            r
        };
        if let Some((tile_set_type, path)) = self.tiles_info.get(&tile_type) {
            match tile_set_type {
                TileSetType::Simple => Some(TextureData::new(path.clone())),
                TileSetType::Complex => {
                    if check([2,0,2,0,0,1,2,1,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 0, 0));
                    }
                    // top right
                    if check([2,0,2,1,0,0,2,1,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 2*TILE_SIZE, 0));
                    }
                    // bottom left
                    if check([2,1,2,0,0,1,2,0,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 0, 2*TILE_SIZE));
                    }
                    // bottom right
                    if check([2,1,2,1,0,0,2,0,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 2*TILE_SIZE, 2*TILE_SIZE));
                    }
                    // top side
                    if check([2,0,2,1,0,1,2,1,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, TILE_SIZE, 0));
                    }
                    // left side
                    if check([2,1,2,0,0,1,0,1,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 0, TILE_SIZE));
                    }
                    // bottom side
                    if check([2,1,2,1,0,1,2,0,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, TILE_SIZE, 2*TILE_SIZE));
                    }
                    // right side
                    if check([2,1,2,1,0,0,2,1,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 2*TILE_SIZE, TILE_SIZE));
                    }
                    // middle
                    if check([2,1,2,1,0,1,2,1,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, TILE_SIZE, TILE_SIZE));
                    }

                    Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, TILE_SIZE, TILE_SIZE ))
                }
                TileSetType::Full => {
                    // weird casework upam da ne zajebem, glej grass_full.png za layout
                    // 0 - empty, 1 - fulll, 2 - whatever

                    // spodna dva cudna
                    if check([1,1,0,1,0,1,0,1,1], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 2*TILE_SIZE, 6*TILE_SIZE));
                    }
                    if check([0,1,1,1,0,1,1,1,0], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 3*TILE_SIZE, 6*TILE_SIZE));
                    }


                    // 2x2 kvadrat desno sredej
                    if check([0,1,0,1,0,1,1,1,0], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 4*TILE_SIZE, 2*TILE_SIZE));
                    }
                    if check([0,1,1,1,0,1,0,1,0], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 5*TILE_SIZE, 2*TILE_SIZE));
                    }
                    if check([1,1,0,1,0,1,0,1,0], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 4*TILE_SIZE, 3*TILE_SIZE));
                    }
                    if check([0,1,0,1,0,1,0,1,1], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 5*TILE_SIZE, 3*TILE_SIZE));
                    }
                    
                    // ta lev 2x2
                    if check([2,1,1,0,0,1,2,1,0], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 0, 4*TILE_SIZE));
                    }
                    if check([1,1,2,1,0,0,0,1,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, TILE_SIZE, 4*TILE_SIZE));
                    }
                    if check([2,1,0,0,0,1,2,1,1], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 0, 5*TILE_SIZE));
                    }
                    if check([0,1,2,1,0,0,1,1,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, TILE_SIZE, 5*TILE_SIZE));
                    }

                    // sredinski 2x2
                    if check([2,0,2,1,0,1,1,1,0], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 2*TILE_SIZE, 4*TILE_SIZE));
                    }
                    if check([2,0,2,1,0,1,0,1,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 3*TILE_SIZE, 4*TILE_SIZE));
                    }
                    if check([2,1,0,1,0,1,2,0,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 2*TILE_SIZE, 5*TILE_SIZE));
                    }
                    if check([0,1,2,1,0,1,2,0,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 3*TILE_SIZE, 5*TILE_SIZE));
                    }

                    // tist 2x2 zgor desno
                    if check([1,1,1,1,0,1,1,1,0], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 4*TILE_SIZE, 0));
                    }
                    if check([1,1,1,1,0,1,0,1,1], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE,5*TILE_SIZE, 0));
                    }
                    if check([1,1,0,1,0,1,1,1,1], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 4*TILE_SIZE, TILE_SIZE));
                    }
                    if check([0,1,1,1,0,1,1,1,1], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 5*TILE_SIZE, TILE_SIZE));
                    }
                    // zadn stolpec prvi stirje
                    if check([0,1,1,1,0,1,0,1,1], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 6*TILE_SIZE, 0));
                    }
                    if check([0,1,0,1,0,1,1,1,1], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 6*TILE_SIZE, TILE_SIZE));
                    }
                    if check([1,1,0,1,0,1,1,1,0], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 6*TILE_SIZE, 2*TILE_SIZE));
                    }
                    if check([1,1,1,1,0,1,0,1,0], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 6*TILE_SIZE, 3*TILE_SIZE));
                    }

                    // spodn kvadrat brez robov
                    // top left
                    if check([2,0,2,0,0,1,2,1,0], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 4*TILE_SIZE, 4*TILE_SIZE));
                    }
                    // top right
                    if check([2,0,2,1,0,0,0,1,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 6*TILE_SIZE, 4*TILE_SIZE));
                    }
                    // bottom left
                    if check([2,1,0,0,0,1,2,0,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 4*TILE_SIZE, 6*TILE_SIZE));
                    }
                    // bottom right
                    if check([0,1,2,1,0,0,2,0,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 6*TILE_SIZE, 6*TILE_SIZE));
                    }
                    // top side
                    if check([2,0,2,1,0,1,0,1,0], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 5*TILE_SIZE, 4*TILE_SIZE));
                    }
                    // left side
                    if check([2,1,0,0,0,1,0,1,0], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 4*TILE_SIZE, 5*TILE_SIZE));
                    }
                    // bottom side
                    if check([0,1,0,1,0,1,2,0,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 5*TILE_SIZE, 6*TILE_SIZE));
                    }
                    // right side
                    if check([0,1,2,1,0,0,0,1,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 6*TILE_SIZE, 5*TILE_SIZE));
                    }
                    // middle
                    if check([0,1,0,1,0,1,0,1,0], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 5*TILE_SIZE, 5*TILE_SIZE));
                    }


                    // isolated
                    if check([2, 0, 2, 0, 0, 0, 2, 0, 2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 3*TILE_SIZE, 3*TILE_SIZE));
                    }
                    // middle no sides
                    if check([2, 1, 2, 0, 0, 0, 2, 1, 2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 3*TILE_SIZE, TILE_SIZE));
                    }
                    if check([2,0,2,1,0,1,2,0,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, TILE_SIZE, 3*TILE_SIZE));
                    }
                    // top/ right of no sides
                    if check([2,0,2,0,0,0,2,1,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 3*TILE_SIZE, 0));
                    }
                    if check([2,0,2,1,0,0,2,0,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 2*TILE_SIZE, 3*TILE_SIZE));
                    }
                    // bottom/left of no sides
                    if check([2,1,2,0,0,0,2,0,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 3*TILE_SIZE, 2*TILE_SIZE));
                    }
                    if check([2,0,2,0,0,1,2,0,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 0, 3*TILE_SIZE));
                    }

                    // regular stuff
                    // top left
                    if check([2,0,2,0,0,1,2,1,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 0, 0));
                    }
                    // top right
                    if check([2,0,2,1,0,0,2,1,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 2*TILE_SIZE, 0));
                    }
                    // bottom left
                    if check([2,1,2,0,0,1,2,0,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 0, 2*TILE_SIZE));
                    }
                    // bottom right
                    if check([2,1,2,1,0,0,2,0,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 2*TILE_SIZE, 2*TILE_SIZE));
                    }
                    // top side
                    if check([2,0,2,1,0,1,2,1,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, TILE_SIZE, 0));
                    }
                    // left side
                    if check([2,1,2,0,0,1,0,1,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 0, TILE_SIZE));
                    }
                    // bottom side
                    if check([2,1,2,1,0,1,2,0,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, TILE_SIZE, 2*TILE_SIZE));
                    }
                    // right side
                    if check([2,1,2,1,0,0,2,1,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 2*TILE_SIZE, TILE_SIZE));
                    }
                    // middle
                    if check([2,1,2,1,0,1,2,1,2], neighbours) {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, TILE_SIZE, TILE_SIZE));
                    }

                    
                    Some(TextureData::new(path.clone()))
                }
                TileSetType::FullVariants => {
                    // Logic for full variants
                    Some(TextureData::new(path.clone()))
                }
            }
        } else {
            None
        }
    }
}