// button nej sprejme neko callback funkcijo ki se bo izvedla ko kliknemo na gumb
// button bo imel tudi svoj text / sliko
// button bo imel svojo pozicijo in velikost
// probi dodat se dropdown menu

// aja life pro tip
// za uporabo drugih modulov v projektu uporabi use crate::modul::modul
// amapk mora bit dodan v main.rs kot `mod modul;` lhk pogledas

use crate::texture_data::TextureData;
use sdl2::{event::Event, rect::Rect};

pub struct Button<F>
where
    F: FnMut(),
{
    //funkcija
    pub function: F,
    //text & texture
    pub text: Option<String>,
    pub texture: Option<TextureData>,
    //position
    pub position: Rect,
}

impl<F: FnMut()> Button<F> {
    pub fn new(fun: F) -> Button<F> {
        Button {
            function: fun,
            text: None,
            texture: None,
            position: Rect::new(0, 0, 50, 50),
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 165, 0));
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
}
