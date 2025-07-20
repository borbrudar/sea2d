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
        if let Some((tile_set_type, path)) = self.tiles_info.get(&tile_type) {
            match tile_set_type {
                TileSetType::Simple => Some(TextureData::new(path.clone())),
                TileSetType::Complex => {
                    // middle tile
                    if neighbours[0][1] && neighbours[1][0] && neighbours[1][2] && neighbours[2][1] { 
                        println!("Using full tile for complex tile: {}", path);
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, TILE_SIZE, TILE_SIZE ));
                    }
                    // edge tiles
                    // top 
                    if neighbours[2][1] && neighbours[1][0] && neighbours[1][2] {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, TILE_SIZE, 0));
                    }
                    // left
                    if neighbours[1][2] && neighbours[0][1] && neighbours[2][1] {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 0, TILE_SIZE));
                    }
                    // bottom
                    if neighbours[0][1] && neighbours[1][0] && neighbours[1][2] {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, TILE_SIZE, 2*TILE_SIZE));
                    }
                    // right
                    if neighbours[1][0] && neighbours[0][1] && neighbours[2][1] {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 2*TILE_SIZE, TILE_SIZE));
                    }

                    // left top corner
                    if neighbours[1][2] && neighbours[2][1] && neighbours[2][2]{
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 0, 0));
                    }
                    // right top corner
                    if neighbours[1][0] && neighbours[2][1] && neighbours[2][0] {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 2*TILE_SIZE, 0));
                    }
                    // left bottom corner
                    if neighbours[0][1] && neighbours[1][2] && neighbours[0][2] {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 0, 2*TILE_SIZE));
                    }
                    // right bottom corner
                    if neighbours[1][0] && neighbours[0][1] && neighbours[0][0] {
                        return Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 2*TILE_SIZE, 2*TILE_SIZE));
                    }   

                    Some(TextureData::new_full(path.clone(), TILE_SIZE, TILE_SIZE, 0,0))
                }
                TileSetType::Full => {
                    // Logic for full tiles
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