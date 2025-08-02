/// Modul za izstrelke v igri.
///
/// Izstrelek (`Projectile`) predstavlja entiteto, ki se giblje po ravni in lahko
/// poškoduje igralca ali sovražnike, odvisno od tega, kdo ga je izstrelil.
use crate::{
    entities::{animated_texture::AnimatedTexture, camera::Camera, enemy::Enemy, player::Player},
    environment::{aabb::AABB, level::Level, tile_type::TileType},
};

/// Struktura, ki predstavlja izstrelek.
///
/// Izstrelek ima položaj, velikost, hitrost, smer gibanja (v radianih),
/// lahko ima animirano teksturo, zadetno škatlo (hitbox) in oznako, ali ga je
/// izstrelil igralec.
#[derive(Debug, Clone)]
pub struct Projectile {
    /// X-koordinata izstrelka.
    pub x: f64,
    /// Y-koordinata izstrelka.
    pub y: f64,
    /// Velikost izstrelka (širina in višina).
    pub size: u32,
    /// Hitrost gibanja izstrelka (v enotah na sekundo).
    pub speed: f64,
    /// Smer gibanja v radianih (0 = desno, π/2 = gor itd.).
    pub direction: f64,
    /// Animirana tekstura izstrelka.
    pub texture: Option<AnimatedTexture>,
    /// Zadetna škatla izstrelka za preverjanje trkov.
    pub hitbox: AABB,
    /// Ali je izstrelek izstrelil igralec.
    pub fired_by_player: bool,
}

impl Projectile {
    /// Ustvari nov izstrelek z danim položajem, velikostjo, smerjo in označbo, kdo ga je izstrelil.
    ///
    /// Privzeta hitrost je 400.
    pub fn new(x: f64, y: f64, size: u32, direction: f64, fired_by_player: bool) -> Projectile {
        Projectile {
            x,
            y,
            speed: 400.0,
            size,
            direction,
            texture: None,
            hitbox: AABB::new(x, y, size, size),
            fired_by_player,
        }
    }

    /// Naloži animirano teksturo izstrelka.
    ///
    /// Pot do teksture je trenutno nastavljena na `"resources/textures/projectile.png"`.
    pub fn load_projectile_texture<'a>(
        &mut self,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        texture_map: &mut std::collections::HashMap<String, sdl2::render::Texture<'a>>,
    ) {
        let path = "resources/textures/projectile.png".to_string();
        self.texture = Some(AnimatedTexture::new(0.1));
        self.texture.as_mut().unwrap().load_animation(
            path,
            0,
            0,
            16,
            16,
            1,
            texture_creator,
            texture_map,
        );
        self.hitbox.x = self.x;
        self.hitbox.y = self.y;
    }

    /// Posodobi položaj izstrelka glede na časovni zamik `dt`.
    ///
    /// Upošteva hitrost in smer gibanja.
    pub fn update(&mut self, dt: f64) {
        self.x += self.speed * dt * self.direction.cos();
        self.y += self.speed * dt * self.direction.sin();
        self.hitbox.x = self.x;
        self.hitbox.y = self.y;
    }

    /// Izračuna kot (v radianih) med izhodiščem in ciljno točko.
    ///
    /// Uporablja funkcijo `atan2` za pravilno določanje kvadranta.
    pub fn calculate_direction(start_x: f64, start_y: f64, target_x: f64, target_y: f64) -> f64 {
        let delta_x = target_x - start_x;
        let delta_y = target_y - start_y;
        delta_y.atan2(delta_x)
    }

    /// Nariše izstrelek na platno.
    ///
    /// Če ima izstrelek teksturo, jo izriše; sicer nariše rdeč kvadrat.
    pub fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture_map: &std::collections::HashMap<String, sdl2::render::Texture>,
        camera: &Camera,
    ) {
        if let Some(ref texture) = self.texture {
            texture.draw(
                canvas,
                texture_map,
                self.x - camera.x,
                self.y - camera.y,
                self.size as u32,
                self.size as u32,
            );
        } else {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
            canvas
                .fill_rect(sdl2::rect::Rect::new(
                    (self.x - camera.x) as i32,
                    (self.y - camera.y) as i32,
                    self.size as u32,
                    self.size as u32,
                ))
                .unwrap();
        }
    }

    /// Reši trke izstrelka z okoljem, sovražniki ali igralcem.
    ///
    /// Če pride do trka:
    /// - s trdnim okoljem → `true`;
    /// - s sovražnikom (če ga je izstrelil igralec) → zmanjša zdravje sovražnika in vrne `true`;
    /// - z igralcem (če ga ni izstrelil igralec) → zmanjša zdravje igralca in vrne `true`.
    ///
    /// Vrne `true`, če je prišlo do trka.
    pub fn resolve_collision(
        &self,
        level: &Level,
        enemies: &mut Vec<Enemy>,
        player: &mut Player,
    ) -> bool {
        let mut ret = false;

        for tile in level.check_collision(&self.hitbox) {
            if tile.bounding_box.is_none() {
                continue;
            }
            if tile.tile_type == TileType::Water {
                continue;
            }
            ret = true;
            break;
        }

        if self.fired_by_player {
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
