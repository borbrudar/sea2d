use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sdl2::image::LoadTexture;

pub struct ThreadSafeTexture<'a> {
    texture: Arc<Mutex<Option<Texture<'a>>>>, // Wrap the texture in Arc<Mutex<_>> for thread safety
}

unsafe impl<'a> Send for ThreadSafeTexture<'a> {}

impl<'a> ThreadSafeTexture<'a> {
    // Constructor that initializes the texture with None inside a Mutex
    pub fn new() -> Self {
        ThreadSafeTexture {
            texture: Arc::new(Mutex::new(None)),
        }
    }

    // Load a texture using the TextureCreator, this method locks the Mutex before accessing the texture
    pub fn load_texture(&self, texture_creator: &'a TextureCreator<WindowContext>, path: &str) -> Result<(), String> {
        let mut texture_lock = self.texture.lock().unwrap(); // Lock the Mutex
        let texture = texture_creator.load_texture(path).map_err(|e| e.to_string())?;
        *texture_lock = Some(texture); // Set the texture inside the Mutex
        Ok(())
    }

    // Render the texture on the given canvas, this method locks the Mutex before accessing the texture
    pub fn render(&self, canvas: &mut WindowCanvas, x: i32, y: i32, width: u32, height: u32) -> Result<(), String> {
        let texture_lock = self.texture.lock().unwrap(); // Lock the Mutex

        if let Some(texture) = &*texture_lock {
            // Copy the texture to the canvas (rendering it at specified position)
            canvas.copy(texture, None, Some(Rect::new(x, y, width, height)))?;
            Ok(())
        } else {
            Err("Texture not loaded".into())
        }
    }
    // Example of modifying the texture (e.g., changing the color or other attributes)
    /*
    pub fn modify_texture(&self) {
        let texture_lock = self.texture.lock().unwrap(); // Lock the Mutex
        if let Some(_) = &*texture_lock {
            // Here we could perform operations that modify the texture
            println!("Texture is being modified in thread {:?}", thread::current().id());
        }
    }
    */

    // Returns a clone of the Arc to allow sharing ownership
    pub fn arc_clone(&self) -> ThreadSafeTexture<'a> {
        ThreadSafeTexture {
            texture: Arc::clone(&self.texture),
        }
    }
}
