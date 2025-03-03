use serde::{Deserialize,Serialize};
use sdl2::render::TextureCreator;
use sdl2::render::Texture;
use sdl2::image::LoadTexture;


#[derive(Serialize,Deserialize,Debug,Clone,Eq,PartialEq,Hash)]
pub struct TextureData {
    pub path : String,
    pub width : u32,
    pub height : u32,
    pub x : i32,
    pub y : i32,
}

impl<'a> TextureData{
    pub fn new(path : String) -> TextureData{
        TextureData{
            path,
            width : 0,
            height : 0,
            x : 0,
            y : 0,
        }
    }
    
    pub fn load_texture(& mut self,texture_creator : & 'a TextureCreator<sdl2::video::WindowContext>, texture_map : & mut std::collections::HashMap<TextureData,Texture<'a>>){
        match texture_map.get(&self){
            Some(..) => (),
            None => {
                //println!("Path: {:?}",self.path);
                let loaded_texture = texture_creator.load_texture(&self.path);
                match loaded_texture{
                    Ok(texture) => {
                        if self.width == 0 || self.height == 0{
                            let query = texture.query();
                            self.width = query.width;
                            self.height = query.height;
                        }
                        texture_map.insert(self.clone(),texture);
                    },
                    Err(..) => println!("couldnt load texture for some reason")
                }
            }
        }
    }
    pub fn draw(&self, canvas : &mut sdl2::render::Canvas<sdl2::video::Window>, texture_map : &std::collections::HashMap<TextureData,Texture>, x : i32, y : i32, width : u32, height : u32) -> Result<(),String>{
        match texture_map.get(&self.clone()){
            Some(texture) => {
                canvas.copy(texture,sdl2::rect::Rect::new(self.x,self.y,self.width,self.height),sdl2::rect::Rect::new(x,y,width,height)).map_err(|e| e.to_string())
            },
            None => Err("Texture not loaded".into())
        }
    }
}