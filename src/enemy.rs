use sdl2::{render::{Canvas, Texture}, video::Window};

use crate::{aabb::AABB, animated_texture::AnimatedTexture, camera::Camera};



pub struct Enemy{
    pub x : f64,
    pub y : f64,
    pub animation_data : Option<AnimatedTexture>,
    pub size : u32,
    pub hitbox : AABB,
}

impl Enemy{
    pub fn new() ->  Enemy{
        Enemy{
            x : 50.,
            y : 50., 
            animation_data : None,
            size : 50,
            hitbox : AABB::new(50.,50.,50,50)
        }
    }

    pub fn draw(&self,canvas : &mut Canvas<Window>, texture_map : &std::collections::HashMap<String,Texture>, camera : &Camera){ 
        match self.animation_data {
            Some(ref animation_data) => {
                animation_data.draw(canvas,texture_map,self.x-camera.x,self.y-camera.y,self.size,self.size);
            },
            None => {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(255,192,203));
                canvas.fill_rect(sdl2::rect::Rect::new((self.x -camera.x) as i32,(self.y-camera.y)as i32,self.size,self.size)).unwrap();
            }
        }
    }

    pub fn update(&mut self, dt : f64){
        match self.animation_data {
            Some(ref mut animation_data) => {
                animation_data.update(dt);
            },
            None => ()
        }
    }
}