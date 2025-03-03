use std::collections::HashMap;

use sdl2::render::{Texture, TextureCreator};

use crate::{camera::Camera, texture_data::TextureData, tile::Tile};

pub struct Level{
    pub tiles : Vec<Tile>,
    pub width : u32,
    pub height : u32,
}

impl<'a> Level{
    pub fn new(width : u32, height : u32, texture_creator : &'a TextureCreator<sdl2::video::WindowContext>, texture_map : &mut HashMap<String,Texture<'a>>) -> Level{
        let mut tiles = Vec::new();
        for x in 0..width{
            for y in 0..height{
                tiles.push(Tile::new(x as i32 * 32,y as i32 * 32,32));
                tiles.last_mut().unwrap().texture_data = Some(TextureData::new("resources/textures/tile.png".to_string()));
                tiles.last_mut().unwrap().texture_data.as_mut().unwrap().load_texture(&texture_creator, texture_map);
            }
        }
        Level{
            tiles,
            width,
            height,
        }
    }

    pub fn draw(&self,canvas : &mut sdl2::render::Canvas<sdl2::video::Window>, texture_map : &std::collections::HashMap<String,sdl2::render::Texture>, camera : &Camera){
        for tile in &self.tiles{
            tile.draw(canvas,texture_map,camera);
        }
    }
}
