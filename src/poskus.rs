//alternativni design za button (človeku pač zmanjka imen in začne uporablat več jezikov idk)

use crate::game::GameState;
use crate::texture_data::TextureData;
use sdl2::pixels::Color as RGB;
use sdl2::{event::Event, rect::Point, rect::Rect};

pub struct Gumb {
    //stanje, v katero se igra premakne ob pritisku na gumb
    pub stanje: GameState,
    //text and texture
    pub text: Option<String>,
    pub texture: Option<TextureData>,
    //colour
    pub colour: RGB,
    //position
    pub position: Rect,
}

impl Gumb {
    pub fn new(
        stanje: GameState,
        text: Option<String>,
        texture: Option<TextureData>,
        colour: RGB,
        position: Rect,
    ) -> Gumb {
        Gumb {
            stanje,
            text,
            texture,
            colour,
            position,
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(self.colour);
        canvas.fill_rect(self.position).unwrap();
    }

    pub fn handle_event(&mut self, event: &Event) -> Option<GameState> {
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
                Some(self.stanje)
            } else {
                None
            }
        } else {
            None
        }
    }
}
