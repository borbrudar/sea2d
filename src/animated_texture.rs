use serde::{Serialize,Deserialize};

use crate::texture_data::TextureData;


#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct AnimatedTexture{
    pub frames : Vec<TextureData>,
    pub current_frame : usize,
    pub frame_time : f64,
    pub current_time : f64,
}

impl<'a> AnimatedTexture{
    pub fn new(frame_time : f64) -> AnimatedTexture{
        AnimatedTexture{
            frames : Vec::new(),
            current_frame : 0,
            frame_time,
            current_time : 0.0,
        }
    }

    pub fn update(&mut self, dt : f64){
        self.current_time += dt;
        if self.current_time >= self.frame_time{
            self.current_time = 0.0;
            self.current_frame += 1;
            if self.current_frame >= self.frames.len(){
                self.current_frame = 0;
            }
        }
    }

    pub fn draw(&self, canvas : &mut sdl2::render::Canvas<sdl2::video::Window>, texture_map : &std::collections::HashMap<String,sdl2::render::Texture>, x : i32, y : i32, w : u32, h : u32){
        self.frames[self.current_frame].draw(canvas,texture_map,x,y,w,h).unwrap();
    }

    pub fn load_animation(&mut self, path : String,
        start_x : i32, start_y : i32, width : u32, height : u32,
        frame_count : u32,
        texture_creator : &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>, 
        texture_map : &mut std::collections::HashMap<String,sdl2::render::Texture<'a>>){

        for i in 0..frame_count{
            let mut frame = TextureData::new(path.clone());
            frame.x = start_x + (i as i32) * width as i32;
            frame.y = start_y;
            frame.width = width;
            frame.height = height;
            frame.load_texture(texture_creator,texture_map);
            self.frames.push(frame);
        }
    }
}