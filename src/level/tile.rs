use crate::level::{aabb::AABB, texture_data::TextureData, tile_type::TileType};
use crate::networking::shared::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::player::camera::Camera;

#[derive(Debug, Clone)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub size: u32,
    pub texture_data: Option<TextureData>,
    pub _tile_type: TileType,
    pub bounding_box: Option<AABB>,
}

impl Tile {
    pub fn new(
        x: i32,
        y: i32,
        size: u32,
        _tile_type: TileType,
        bounding_box: Option<AABB>,
    ) -> Tile {
        Tile {
            x,
            y,
            size,
            texture_data: None,
            _tile_type,
            bounding_box,
        }
    }

    pub fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture_map: &std::collections::HashMap<String, sdl2::render::Texture>,
        camera: &Camera,
    ) {
        //let scaled_w = camera.apply_zoom(self.size as f32) as u32;
        //let scaled_h = camera.apply_zoom(self.size as f32) as u32;
        let scaled_w = self.size;
        let scaled_h = self.size;
        // cull everyhting outside the frame
        if self.x as f64 - camera.x + (scaled_w as f64) < 0.0
            || self.y as f64 - camera.y + (scaled_h as f64) < 0.0
            || self.x as f64 - camera.x > SCREEN_WIDTH as f64
            || self.y as f64 - camera.y > SCREEN_HEIGHT as f64
        {
            return;
        }
        match self.texture_data {
            Some(ref texture_data) => {
                let res = texture_data.draw(
                    canvas,
                    texture_map,
                    self.x as f64 - camera.x,
                    self.y as f64 - camera.y,
                    scaled_w,
                    scaled_h,
                );
                match res {
                    Err(..) => {
                        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                        canvas
                            .fill_rect(sdl2::rect::Rect::new(
                                (self.x as f64 - camera.x) as i32,
                                (self.y as f64 - camera.y) as i32,
                                scaled_w,
                                scaled_h,
                            ))
                            .unwrap();
                    }
                    Ok(..) => (),
                }
            }
            None => {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                canvas
                    .fill_rect(sdl2::rect::Rect::new(
                        (self.x as f64 - camera.x) as i32,
                        (self.y as f64 - camera.y) as i32,
                        scaled_w,
                        scaled_h,
                    ))
                    .unwrap();
            }
        }
    }
}
