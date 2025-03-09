//use sdl2::{event::Event, keyboard::Keycode};


pub struct Camera{
    pub x : i32,
    pub y : i32,
    pub width : u32,
    pub height : u32,
    //zoom : f32,
}

impl Camera{
    pub fn new(x : i32, y : i32, width : u32, height : u32) -> Camera{
        Camera{
            x,
            y,
            width,
            height,
            //zoom : 1.0
        }
    }
    /*
    pub fn apply_zoom(&self, size: f32) -> f32 {
        size * self.zoom
    }
    
    pub fn adjust_zoom(&mut self, delta: f32) {
        self.zoom += delta;
        // Clamp zoom level to a reasonable range
        if self.zoom < 0.1 {
            self.zoom = 0.1; // Prevent zooming out too much
        }
        if self.zoom > 3.0 {
            self.zoom = 3.0; // Prevent zooming in too much
        }
    }
    pub fn handle_zoom(&mut self, event: &Event) {
        match event {
            Event::MouseWheel { y, .. } => {
                // Zoom in (positive scroll) or out (negative scroll)
                let zoom_delta = if *y > 0 { 0.1 } else { -0.1 };
                self.adjust_zoom(zoom_delta);
            }
            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                // Zoom in (using arrow key)
                self.adjust_zoom(0.1);
            }
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                // Zoom out (using arrow key)
                self.adjust_zoom(-0.1);
            }
            _ => {}
        }
    }
    */
    pub fn move_camera(&mut self, x : i32, y : i32){
        self.x += x;
        self.y += y;
    }
    pub fn set_camera(&mut self, x : i32, y : i32){
        self.x = x;
        self.y = y;
    }
    pub fn get_camera_rect(&self) -> sdl2::rect::Rect{
        sdl2::rect::Rect::new(self.x,self.y,self.width,self.height)
    }
    pub fn get_camera_rect_mut(&mut self) -> sdl2::rect::Rect{
        sdl2::rect::Rect::new(self.x,self.y,self.width,self.height)
    }
    pub fn get_camera_center(&self) -> (i32,i32){
        (self.x + self.width as i32 / 2,self.y + self.height as i32 / 2)
    }
    pub fn get_camera_center_rect(&self) -> sdl2::rect::Rect{
        sdl2::rect::Rect::new(self.x + self.width as i32 / 2,self.y + self.height as i32 / 2,1,1)
    }
    pub fn get_camera_center_rect_mut(&mut self) -> sdl2::rect::Rect{
        sdl2::rect::Rect::new(self.x + self.width as i32 / 2,self.y + self.height as i32 / 2,1,1)
    }
    pub fn get_camera_center_rect_size(&self, size : u32) -> sdl2::rect::Rect{
        sdl2::rect::Rect::new(self.x + self.width as i32 / 2 - size as i32 / 2,self.y + self.height as i32 / 2 - size as i32 / 2,size,size)
    }
    pub fn get_camera_center_rect_size_mut(&mut self, size : u32) -> sdl2::rect::Rect{
        sdl2::rect::Rect::new(self.x + self.width as i32 / 2 - size as i32 / 2  ,self.y + self.height as i32 / 2 - size as i32 / 2,size,size)
    }
}