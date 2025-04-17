// button nej sprejme neko callback funkcijo ki se bo izvedla ko kliknemo na gumb
// button bo imel tudi svoj text / sliko
// button bo imel svojo pozicijo in velikost
// probi dodat se dropdown menu

// aja life pro tip
// za uporabo drugih modulov v projektu uporabi use crate::modul::modul
// amapk mora bit dodan v main.rs kot `mod modul;` lhk pogledas

use crate::game;
use crate::texture_data::TextureData;
use sdl2::pixels::Color as RGB;
use sdl2::{event::Event, rect::Rect};

pub struct Button<'a> {
    //funkcija
    pub function: Box<dyn FnMut() + 'a>,
    //text & texture
    pub text: Option<String>,
    pub texture: Option<TextureData>,
    //colour
    pub colour: RGB,
    //position
    pub position: Rect,
}

// Rect::new(0, 0, 50, 50)

impl<'a> Button<'a> {
    pub fn new(
        fun: Box<dyn FnMut() + 'a>,
        line: Option<String>,
        tex: Option<TextureData>,
        col: RGB,
        pos: Rect,
    ) -> Button<'a> {
        Button {
            function: fun,
            text: line,
            texture: tex,
            colour: col,
            position: pos,
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(self.colour);
        canvas.fill_rect(self.position).unwrap();
    }

    pub fn handle_event(&mut self, event: &Event) {
        if let Event::MouseButtonDown {
            timestamp,
            window_id,
            which,
            mouse_btn,
            clicks,
            x,
            y,
        } = event
        {
            if self.position.contains_point((*x, *y)) {
                (self.function)();
            }
        }
    }

    pub fn create_pause_button<'b>(g: &'b mut game::Game) -> Button<'b> {
        Button::new(
            Box::new(|| g.pause()),
            Some(String::from("Pause")),
            None,
            RGB::RGB(255, 0, 0),
            Rect::new(0, 0, 50, 50),
        )
    }
}

pub struct HealthBar {
    pub position: Rect,
    pub health: u32,
    pub max_health: u32,
}

impl HealthBar {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> HealthBar {
        HealthBar {
            position: Rect::new(x, y, w, h),
            health: 100,
            max_health: 100,
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(RGB::RGB(255, 0, 0));
        canvas.fill_rect(self.position).unwrap();
    }
}
pub struct Badge {
    pub position: Rect,
    pub texture: TextureData,
}
