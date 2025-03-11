use serde::{Serialize,Deserialize};

use crate::texture_data::TextureData;


#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub enum AnimationType{
    Loop,
    PingPong,
}

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
pub struct AnimatedTexture{
    pub frames : Vec<TextureData>,
    pub current_frame : i32,
    pub previous_frame : i32,
    pub frame_time : f64,
    pub current_time : f64,
    pub animation_type : AnimationType,
}

impl<'a> AnimatedTexture{
    pub fn new(frame_time : f64) -> AnimatedTexture{
        AnimatedTexture{
            frames : Vec::new(),
            current_frame : 0,
            previous_frame : 0,
            frame_time,
            current_time : 0.0,
            animation_type : AnimationType::Loop,
        }
    }

    pub fn update(&mut self, dt : f64){
        self.current_time += dt;
        if self.current_time >= self.frame_time{
            self.current_time = 0.0;
            let dir = self.current_frame as i32 - self.previous_frame as i32;
            self.previous_frame = self.current_frame;
            if dir != 0{
                self.current_frame += dir;
            }else{
                self.current_frame += 1;
            }
            if self.current_frame as usize >= self.frames.len() || self.current_frame < 0{
                match self.animation_type {
                    AnimationType::Loop => {
                        self.current_frame = 0;
                    },
                    AnimationType::PingPong => {
                        self.current_frame = self.frames.len() as i32 - 2;
                    }
                }
            }

        }
    }

    pub fn draw(&self, canvas : &mut sdl2::render::Canvas<sdl2::video::Window>, texture_map : &std::collections::HashMap<String,sdl2::render::Texture>, x : f64, y : f64, w : u32, h : u32){
        self.frames[self.current_frame as usize].draw(canvas,texture_map,x,y,w,h).unwrap();
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