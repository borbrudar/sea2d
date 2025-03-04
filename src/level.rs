use std::collections::HashMap;

use ::image::RgbaImage;
use sdl2::render::{Texture, TextureCreator};

use crate::{camera::Camera, texture_data::TextureData, tile::Tile};

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
    
        for y in 0..height{
            for x in 0..width{
                let pixel = img.get_pixel(x, y);

                //println!("Pixel: {:?}",pixel);
                if pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0{
                    self.tiles.push(Tile::new(x as i32 * 32, y as i32 * 32, 32));
                    self.tiles.last_mut().unwrap().texture_data = Some(TextureData::new("resources/textures/tile.png".to_string()));
                    self.tiles.last_mut().unwrap().texture_data.as_mut().unwrap().load_texture(&texture_creator, texture_map);
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
