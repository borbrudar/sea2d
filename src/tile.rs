use crate::aabb::AABB;
use crate::shared::{SCREEN_HEIGHT,SCREEN_WIDTH};
use crate::{camera::Camera, texture_data::TextureData};
use crate::tile_type::TileType;

pub struct Tile{
    pub x : f64,
    pub y : f64,
    pub size : u32,
    pub texture_data : Option<TextureData>,
    pub _tile_type : TileType,
    pub bounding_box : Option<AABB>,
}

impl Tile{
    pub fn new(x : f64, y : f64, size : u32, _tile_type : TileType, bounding_box : Option<AABB>) -> Tile{
        Tile{
            x,
            y,
            size,
            texture_data : None,
            _tile_type,
            bounding_box,
        }
    }

    pub fn draw(&self,canvas : &mut sdl2::render::Canvas<sdl2::video::Window>, texture_map : &std::collections::HashMap<String,sdl2::render::Texture>, camera : &Camera){
        //let scaled_w = camera.apply_zoom(self.size as f32) as u32;
        //let scaled_h = camera.apply_zoom(self.size as f32) as u32;
        let scaled_w = self.size;
        let scaled_h = self.size;
        if self.x-camera.x+(scaled_w as f64)< 0.0 || self.y-camera.y+(scaled_h as f64) < 0.0 || self.x-camera.x > SCREEN_WIDTH as f64 || self.y-camera.y > SCREEN_HEIGHT as f64{
            return;
        }
        match self.texture_data {
            Some(ref texture_data) => {
                let res = texture_data.draw(canvas,texture_map,self.x-camera.x,self.y-camera.y,scaled_w ,scaled_h);
                match res {
                    Err(..) => {
                        canvas.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));                        
                        canvas.fill_rect(sdl2::rect::Rect::new((self.x-camera.x)as i32,(self.y-camera.y) as i32, scaled_w,scaled_h)).unwrap();
                    },
                    Ok(..) => ()
                }
            },
            None => {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(0,0,0));
                canvas.fill_rect(sdl2::rect::Rect::new((self.x-camera.x) as i32,(self.y-camera.y) as i32,scaled_w,scaled_h)).unwrap();
            }
        }
    }
}