use sdl2::{pixels::Color, sys::__int_least32_t};

use crate::camera::Camera;



pub struct AABB{
    pub x : i32,
    pub y : i32,
    pub w : u32,
    pub h : u32,
}


impl AABB{
    pub fn new(x : i32, y : i32, w : u32, h : u32) -> AABB{
        AABB{
            x,
            y,
            w,
            h,
        }
    }
    pub fn draw(&self, canvas : &mut sdl2::render::Canvas<sdl2::video::Window>, color : Color, camera : &Camera){
        canvas.set_draw_color(color);
        canvas.draw_rect(sdl2::rect::Rect::new(self.x-camera.x,self.y-camera.y,self.w,self.h)).unwrap();
    }

    pub fn intersects(&self, other : &AABB) -> bool{
        self.x < other.x + (other.w as i32) &&
        self.x + (self.w as i32) > other.x &&
        self.y < other.y + (other.h as i32) &&
        self.y + (self.h as i32) > other.y
    }
    pub fn contains(&self, other : &AABB) -> bool{
        self.x < other.x &&
        self.x + self.w as i32 > other.x + other.w as i32 &&
        self.y < other.y &&
        self.y + self.h as i32 > other.y + other.h as i32
    }
    pub fn translate(&mut self, x : i32, y : i32){
        self.x += x;
        self.y += y;
    }
    pub fn set_position(&mut self, x : i32, y : i32){
        self.x = x;
        self.y = y;
    }
    pub fn get_position(&self) -> (i32,i32){
        (self.x,self.y)
    }
    pub fn get_center(&self) -> (i32,i32){
        (self.x + self.w as i32 / 2,self.y + self.h as i32 / 2)
    }
    pub fn get_size(&self) -> (u32,u32){
        (self.w,self.h)
    }
    pub fn get_center_position(&self) -> (i32,i32){
        (self.x + self.w as i32/ 2,self.y + self.h as i32 / 2)
    }
    pub fn get_center_position_mut(&mut self) -> (i32,i32){
        (self.x + self.w as i32 / 2,self.y + self.h as __int_least32_t / 2)
    }
    pub fn get_rect(&self) -> sdl2::rect::Rect{
        sdl2::rect::Rect::new(self.x as i32,self.y as i32,self.w as u32,self.h as u32)
    }
    pub fn get_rect_mut(&mut self) -> sdl2::rect::Rect{
        sdl2::rect::Rect::new(self.x as i32,self.y as i32,self.w as u32,self.h as u32)
    }
}