use crate::{camera::Camera, texture_data::TextureData};


pub struct Tile{
    pub x : i32,
    pub y : i32,
    pub size : u32,
    pub texture_data : Option<TextureData>,
}

impl Tile{
    pub fn new(x : i32, y : i32, size : u32) -> Tile{
        Tile{
            x,
            y,
            size,
            texture_data : None
        }
    }

    pub fn draw(&self,canvas : &mut sdl2::render::Canvas<sdl2::video::Window>, texture_map : &std::collections::HashMap<TextureData,sdl2::render::Texture>, camera : &Camera){
        match self.texture_data {
            Some(ref texture_data) => {
                let res = texture_data.draw(canvas,texture_map,self.x-camera.x,self.y-camera.y,self.size,self.size);
                match res {
                    Err(..) => {
                        canvas.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
                        canvas.fill_rect(sdl2::rect::Rect::new(self.x-camera.x,self.y-camera.y,self.size,self.size)).unwrap();
                    },
                    Ok(..) => ()
                }
            },
            None => {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
                canvas.fill_rect(sdl2::rect::Rect::new(self.x-camera.x,self.y-camera.y,self.size,self.size)).unwrap();
            }
        }
    }
}