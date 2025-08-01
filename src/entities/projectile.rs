use crate::{entities::animated_texture::AnimatedTexture, environment::aabb::AABB};



pub struct Projectile{
    pub x: f64,
    pub y: f64,
    pub size : u32,
    pub speed : f64,
    pub direction: f64, // Angle in radians
    pub texture: Option<AnimatedTexture>,
    pub hitbox : AABB,
}

impl Projectile {
    pub fn new(x: f64, y: f64, size : u32, end_x : f64, end_y : f64) -> Projectile {
        Projectile {
            x,
            y,
            speed : 400.0,
            size,
            direction : (end_y - y).atan2(end_x - x), // Calculate angle to target
            texture: None,
            hitbox: AABB::new(x, y, size, size),   
        }
    }

    pub fn load_projectile_texture<'a>(
        &mut self,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        texture_map: &mut std::collections::HashMap<String, sdl2::render::Texture<'a>>,
    ) {
        let path = "resources/textures/projectile.png".to_string(); // Replace with actual path
        self.texture = Some(AnimatedTexture::new(0.1)); // Example frame time
        self.texture.as_mut().unwrap().load_animation(
            path,
            0,
            0,
            16, // Example width
            16, // Example height
            1,  // Example frame count
            texture_creator,
            texture_map,
        );
    } 

    pub fn update(&mut self, dt: f64) {
        self.x += self.speed * dt * self.direction.cos();
        self.y += self.speed * dt * self.direction.sin();
    }

    pub fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture_map: &std::collections::HashMap<String, sdl2::render::Texture>,
    ) {
        if let Some(ref texture) = self.texture {
            texture.draw(canvas, texture_map, self.x, self.y, self.size as u32, self.size as u32);
        } else {
            // Draw a placeholder rectangle if no texture is available
            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
            canvas.fill_rect(sdl2::rect::Rect::new(
                self.x as i32,
                self.y as i32,
                self.size as u32,
                self.size as u32,
            )).unwrap();
        }
    }

}