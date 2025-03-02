


pub struct Camera{
    pub x : i32,
    pub y : i32,
    pub width : u32,
    pub height : u32,
}

impl Camera{
    pub fn new(x : i32, y : i32, width : u32, height : u32) -> Camera{
        Camera{
            x,
            y,
            width,
            height,
        }
    }
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