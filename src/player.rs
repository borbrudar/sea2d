use crate::animated_texture::AnimatedTexture;
use crate::shared::{SCREEN_HEIGHT,SCREEN_WIDTH};
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::texture_data::TextureData;
use sdl2::render::Texture;
use crate::camera::Camera;

pub struct Player{
    pub id : u64,
    pub x : i32,
    pub y : i32,
    pub color : (u8,u8,u8),
    size : u32,
    pub texture_data : Option<TextureData>,
    pub animation_data : Option<AnimatedTexture>
}

impl Player{
    pub fn new(id : u64) -> Player{
        Player{
            id : id,
            x : (SCREEN_WIDTH as i32)/2,
            y : (SCREEN_HEIGHT as i32)/2,
            color : (255,255,255),
            size : 40,
            texture_data : None,
            animation_data : None
        }
    }

    pub fn draw(&self,canvas : &mut Canvas<Window>, texture_map : &std::collections::HashMap<String,Texture>, camera : &Camera){ 
        match self.animation_data {
            Some(ref animation_data) => {
                animation_data.draw(canvas,texture_map,self.x-camera.x,self.y-camera.y,self.size,self.size);
                println!("Drawing animation");
            },
            None => match self.texture_data {
                Some (ref texture_data) => {
                    let res = texture_data.draw(canvas,texture_map,self.x-camera.x,self.y-camera.y,self.size,self.size);
                    match res {
                        Err(..) => {
                            canvas.set_draw_color(sdl2::pixels::Color::RGB(self.color.0,self.color.1,self.color.2));
                            canvas.fill_rect(sdl2::rect::Rect::new(self.x-camera.x,self.y-camera.y,self.size,self.size)).unwrap();
                        },
                        Ok(..) => ()
                    }
                },
                None => {
                    canvas.set_draw_color(sdl2::pixels::Color::RGB(255,192,203));
                    canvas.fill_rect(sdl2::rect::Rect::new(self.x-camera.x,self.y-camera.y,self.size,self.size)).unwrap();
                }
            }
        }
    }
}