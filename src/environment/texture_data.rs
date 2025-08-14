/// Modul za upravljanje s podatki o teksturah.
use sdl2::image::LoadTexture;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use serde::{Deserialize, Serialize};

/// Struktura, ki predstavlja podatke o teksturi.
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct TextureData {
    /// Pot do datoteke teksture.
    pub path: String,
    /// Širina teksture.
    pub width: u32,
    /// Višina teksture.
    pub height: u32,
    /// X koordinata za risanje teksture.
    pub x: u32,
    /// Y koordinata za risanje teksture.
    pub y: u32,
}

impl<'a> TextureData {
    /// Ustvari novo instanco `TextureData` z dano potjo.
    pub fn new(path: String) -> TextureData {
        TextureData {
            path,
            width: 0,
            height: 0,
            x: 0,
            y: 0,
        }
    }
    /// Ustvari novo instanco `TextureData` z dano potjo in velikostjo.
    pub fn new_full(path: String, width: u32, height: u32, x: u32, y: u32) -> TextureData {
        TextureData {
            path,
            width,
            height,
            x,
            y,
        }
    }

    /// Naloži teksturo iz datoteke in jo shrani v `texture_map`.
    pub fn load_texture(
        &mut self,
        texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
        texture_map: &mut std::collections::HashMap<String, Texture<'a>>,
    ) {
        match texture_map.get(&self.path.clone()) {
            Some(tex) => self.size_auto(tex),
            None => {
                //println!("Path: {:?}",self.path);
                let loaded_texture = texture_creator.load_texture(&self.path);
                match loaded_texture {
                    Ok(texture) => {
                        self.size_auto(&texture);
                        texture_map.insert(self.path.clone(), texture);
                    }
                    Err(..) => println!("couldnt load texture for some reason"),
                }
            }
        }
    }

    /// Izriše teksturo na dani koordinati in velikosti.
    pub fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture_map: &std::collections::HashMap<String, Texture>,
        x: f64,
        y: f64,
        width: u32,
        height: u32,
    ) -> Result<(), String> {
        match texture_map.get(&self.path.clone()) {
            Some(texture) => {
                // println!("self x: {}, self y: {}, self width: {}, self height: {}",self.x,self.y,self.width,self.height);
                canvas
                    .copy(
                        texture,
                        sdl2::rect::Rect::new(
                            self.x as i32,
                            self.y as i32,
                            self.width,
                            self.height,
                        ),
                        sdl2::rect::Rect::new(x as i32, y as i32, width, height),
                    )
                    .map_err(|e| e.to_string())
            }
            None => Err("Texture not loaded".into()),
        }
    }

    /// Samodejno nastavi širino in višino teksture, če sta nastavljeni na 0.
    pub fn size_auto(&mut self, texture: &Texture) {
        if self.width == 0 || self.height == 0 {
            let query = texture.query();
            self.width = query.width;
            self.height = query.height;
        }
    }
}
