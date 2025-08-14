/// Modul za osno poravnan omejitveni okvir oz. Axis-Aligned Bounding Box (AABB).
/// Uporablja se za določanje omejitev in prekrivanja objektov v 2D prostoru.
use sdl2::pixels::Color;

use crate::entities::camera::Camera;

/// Struktura, ki predstavlja osno poravnan omejitveni okvir (AABB).
#[derive(Clone, Debug)]
pub struct AABB {
    pub x: f64,
    pub y: f64,
    pub w: u32,
    pub h: u32,
}

impl AABB {
    /// Ustvari nov okvir z danimi koordinatami in dimenzijami.
    pub fn new(x: f64, y: f64, w: u32, h: u32) -> AABB {
        AABB { x, y, w, h }
    }
    /// Izriše okvir na zaslonu.
    pub fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        color: Color,
        camera: &Camera,
    ) {
        canvas.set_draw_color(color);
        canvas
            .draw_rect(sdl2::rect::Rect::new(
                (self.x - camera.x) as i32,
                (self.y - camera.y) as i32,
                self.w,
                self.h,
            ))
            .unwrap();
    }

    /// Preveri, ali se ta okvir prekriva z drugim okvirjem.
    pub fn intersects(&self, other: &AABB) -> bool {
        self.x < other.x + (other.w as f64)
            && self.x + (self.w as f64) > other.x
            && self.y < other.y + (other.h as f64)
            && self.y + (self.h as f64) > other.y
    }
}
