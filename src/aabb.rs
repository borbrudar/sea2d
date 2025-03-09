use sdl2::pixels::Color;

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
}