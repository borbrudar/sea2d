use crate::{entities::{animated_texture::AnimatedTexture, camera::Camera, enemy::Enemy, player::Player}, environment::{aabb::AABB, level::Level, tile_type::TileType}};


#[derive(Debug, Clone)]
pub struct Projectile{
    pub x: f64,
    pub y: f64,
    pub size : u32,
    pub speed : f64,
    pub direction: f64, // Angle in radians
    pub texture: Option<AnimatedTexture>,
    pub hitbox : AABB,
    pub fired_by_player: bool, 
}

impl Projectile {
    pub fn new(x: f64, y: f64, size : u32, direction : f64, fired_by_player : bool) -> Projectile {
        Projectile {
            x,
            y,
            speed : 400.0,
            size,
            direction,
            texture: None,
            hitbox: AABB::new(x, y, size, size),   
            fired_by_player,
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
        self.hitbox.x = self.x;
        self.hitbox.y = self.y;
    }

    pub fn update(&mut self, dt: f64) {
        self.x += self.speed * dt * self.direction.cos();
        self.y += self.speed * dt * self.direction.sin();
        self.hitbox.x = self.x;
        self.hitbox.y = self.y;
    }

    pub fn calculate_direction(start_x : f64, start_y : f64, target_x: f64, target_y: f64) -> f64{
        let delta_x = target_x - start_x;
        let delta_y = target_y - start_y;
        (delta_y / delta_x).atan()
    }

    pub fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture_map: &std::collections::HashMap<String, sdl2::render::Texture>,
        camera : &Camera,
    ) {
        if let Some(ref texture) = self.texture {
            texture.draw(canvas, texture_map, self.x - camera.x, self.y - camera.y, self.size as u32, self.size as u32);
        } else {
            // Draw a placeholder rectangle if no texture is available
            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
            canvas.fill_rect(sdl2::rect::Rect::new(
                (self.x - camera.x) as i32,
                (self.y - camera.y) as i32,
                self.size as u32,
                self.size as u32,
            )).unwrap();
        }
    }

    pub fn resolve_collision(&self, level : &Level, enemies : &mut Vec<Enemy>, player : &mut Player ) -> bool {
        let mut ret = false;
        // check if colliding with level, enemies or player 
        // if so decrease their health and return true
        for tile in level.check_collision(&self.hitbox) {
            if tile.bounding_box.is_some()  == false{
                continue;
            }
            if tile.tile_type == TileType::Water{
                continue;
            }
            ret = true; 
            break;
        }

        if self.fired_by_player{
            for enemy in enemies.iter_mut() {
                if self.hitbox.intersects(&enemy.hitbox) {
                    enemy.health -= 15; 
                    ret = true;
                }
            }
        }

        if self.hitbox.intersects(&player.hitbox) && !self.fired_by_player {
            player.health -= 15; 
            ret = true;
        }

        ret
    }

}