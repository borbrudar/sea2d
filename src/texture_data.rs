use serde::{Deserialize,Serialize};
use sdl2::render::TextureCreator;
use sdl2::render::Texture;
use sdl2::image::LoadTexture;


#[derive(Serialize,Deserialize,Debug,Clone,Eq,PartialEq,Hash)]
pub struct TextureData {
    pub path : String
}

impl<'a> TextureData{
    pub fn new(path : String) -> TextureData{
        TextureData{
            path 
        }
    }
    
    pub fn load_texture<'b>(& 'b mut self,texture_creator : & 'a TextureCreator<sdl2::video::WindowContext>, texture_map : & mut std::collections::HashMap<TextureData,Texture<'a>>){
        match texture_map.get(&self){
            Some(..) => (),
            None => {
                println!("Path: {:?}",self.path);
                let loaded_texture = texture_creator.load_texture(&self.path);
                match loaded_texture{
                    Ok(texture) => {
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
                canvas.copy(texture,None,sdl2::rect::Rect::new(x,y,width,height))
            },
            None => Err("Texture not loaded".into())
        }
    }
}