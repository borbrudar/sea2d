use std::collections::HashMap;

use ::image::RgbaImage;
use sdl2::render::{Texture, TextureCreator};

use crate::{camera::Camera, texture_data::TextureData, tile::Tile, tile_type::{TileType, TileTypeInfo}};

pub struct Level{
    pub tiles : Vec<Tile>,
}

impl<'a> Level{
    pub fn new() -> Level{
        Level{
            tiles : Vec::new()
        }
    }

    pub fn load_from_file(&mut self, path : String, texture_creator : & 'a TextureCreator<sdl2::video::WindowContext>, texture_map : &mut HashMap<String,Texture<'a>>){
        let img = ::image::ImageReader::open(path).expect("Failed to load image").decode().expect("Failed to decode image");
        let img: RgbaImage = img.to_rgba8();
        let (width, height) = img.dimensions();
    
        let tile_size  = 50;
        for y in 0..height{
            for x in 0..width{
                let pixel = img.get_pixel(x, y);
                let pixel = (pixel[0],pixel[1],pixel[2]);


                //println!("Pixel: {:?}",pixel);
                match pixel{
                    TileType::STONE_COLOR => {
                        self.tiles.push(Tile::new(x as i32 * tile_size, y as i32 * tile_size, tile_size as u32, TileType::Grass));
                        self.tiles.last_mut().unwrap().texture_data = Some(TextureData::new("resources/textures/tile.png".to_string()));
                        self.tiles.last_mut().unwrap().texture_data.as_mut().unwrap().load_texture(&texture_creator, texture_map);
                    },
                    TileType::WATER_COLOR => {
                        self.tiles.push(Tile::new(x as i32 * tile_size, y as i32 * tile_size, tile_size as u32, TileType::Water));
                        self.tiles.last_mut().unwrap().texture_data = Some(TextureData::new("resources/textures/water.png".to_string()));
                        self.tiles.last_mut().unwrap().texture_data.as_mut().unwrap().load_texture(&texture_creator, texture_map);
                    },
                    TileType::GRASS_COLOR => {
                        self.tiles.push(Tile::new(x as i32 * tile_size, y as i32 * tile_size, tile_size as u32, TileType::Grass));
                        self.tiles.last_mut().unwrap().texture_data = Some(TextureData::new("resources/textures/grass.png".to_string()));
                        self.tiles.last_mut().unwrap().texture_data.as_mut().unwrap().load_texture(&texture_creator, texture_map);
                    },
                    TileType::SAND_COLOR => {
                        self.tiles.push(Tile::new(x as i32 * tile_size, y as i32 * tile_size, tile_size as u32, TileType::Sand));
                        self.tiles.last_mut().unwrap().texture_data = Some(TextureData::new("resources/textures/sand.jpg".to_string()));
                        self.tiles.last_mut().unwrap().texture_data.as_mut().unwrap().load_texture(&texture_creator, texture_map);
                    },
                    TileType::ROCK_COLOR => {
                        self.tiles.push(Tile::new(x as i32 * tile_size, y as i32 * tile_size, tile_size as u32, TileType::Rock));
                        self.tiles.last_mut().unwrap().texture_data = Some(TextureData::new("resources/textures/rock.png".to_string()));
                        self.tiles.last_mut().unwrap().texture_data.as_mut().unwrap().load_texture(&texture_creator, texture_map);
                    },
                    TileType::TREE_COLOR => {
                        self.tiles.push(Tile::new(x as i32 * tile_size, y as i32 * tile_size, tile_size as u32, TileType::Tree));
                        self.tiles.last_mut().unwrap().texture_data = Some(TextureData::new("resources/textures/tree.png".to_string()));
                        self.tiles.last_mut().unwrap().texture_data.as_mut().unwrap().load_texture(&texture_creator, texture_map);
                    },
                    _ => ()
                }
            }
        }
        
    }


    pub fn draw(&self,canvas : &mut sdl2::render::Canvas<sdl2::video::Window>, texture_map : &std::collections::HashMap<String,sdl2::render::Texture>, camera : &Camera){
        for tile in &self.tiles{
            tile.draw(canvas,texture_map,camera);
        }
    }
}
