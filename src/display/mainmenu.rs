/// Modul za upravljanje glavnega menija igre.
use crate::display::{
    button::{Button, ButtonAction},
    text::Text,
};
use crate::game::GameState;
use crate::networking::shared::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::wfc::overlap::read_highest_level;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::ttf::{self, Sdl2TtfContext};
use sdl2::video::{Window, WindowContext};
use std::collections::HashMap;

/// Struktura, ki predstavlja glavni meni igre.
pub struct MainMenu<'a> {
    pub start_button: Button<'a>,
}

impl<'a> MainMenu<'a> {
    /// Ustvari nov glavni meni z gumbom za začetek igre.
    pub fn new<'b>() -> MainMenu<'b> {
        let dest_rect = Rect::new(
            (SCREEN_WIDTH / 2) as i32 - 75,
            (SCREEN_HEIGHT / 2) as i32 + 80,
            150,
            70,
        );
        let start_button = Button::new(
            ButtonAction::ChangeGameState(GameState::Instructions),
            Some("Start".to_string()),
            None,
            Some(Color::RGB(109, 165, 194)),
            dest_rect,
        );

        MainMenu { start_button }
    }

    /// Izriše glavni meni na danem platnu.
    pub fn draw(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        ttf_context: &ttf::Sdl2TtfContext,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        texture_map: &mut std::collections::HashMap<String, Texture<'a>>,
    ) {
        // Draw background
        let texture = texture_creator
            .load_texture("resources/screenshots/main_menu_background.png")
            .expect("couldn't find texture of mm background");

        // Assuming full-screen background
        let target = Rect::new(0, 0, 800, 600);
        canvas.copy(&texture, None, Some(target)).unwrap();

        // Draw start button
        //draw frame around the button
        let (x, y, width, height) = (
            self.start_button.position.x,
            self.start_button.position.y,
            self.start_button.position.width(),
            self.start_button.position.height(),
        );
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas
            .draw_rect(Rect::new(x - 1, y - 1, width + 2, height + 2))
            .expect("Failed to draw button frame");

        self.start_button
            .draw(canvas, ttf_context, texture_creator, texture_map);

        // Draw high score
        let high_score = Text::new(
            250,
            500,
            40,
            "resources/fonts/manolomono.otf",
            format!(
                "HIGH SCORE: {}",
                read_highest_level("resources/levels/highscore.txt")
            ),
            Color::RGB(255, 255, 255),
        );
        high_score.draw(canvas, ttf_context);
    }
}

/// Struktura, ki predstavlja zaslon z gumbi.
pub struct Screen<'a> {
    pub buttons: Vec<Button<'a>>,
    pub background_png: String,
}

impl<'a> Screen<'a> {
    /// Ustvari nov zaslon z danimi gumbi in ozadjem.
    pub fn new(bts: Vec<Button>, background: String) -> Screen {
        Screen {
            buttons: bts,
            background_png: background,
        }
    }

    /// Izriše zaslon z gumbi na danem platnu.
    pub fn draw(
        &mut self,
        canvas: &mut Canvas<Window>,
        ttf_context: &Sdl2TtfContext,
        texture_creator: &'a TextureCreator<WindowContext>,
        texture_map: &mut HashMap<String, Texture<'a>>,
    ) {
        // Draw background
        let texture = texture_creator
            .load_texture(self.background_png.clone())
            .expect("couldn't find texture of mm background");

        // full-screen background
        let target = Rect::new(0, 0, 800, 600); // Replace with actual screen dimensions
        canvas.copy(&texture, None, Some(target)).unwrap();
        for button in self.buttons.iter_mut() {
            button.draw(canvas, ttf_context, texture_creator, texture_map);
        }
    }
}
