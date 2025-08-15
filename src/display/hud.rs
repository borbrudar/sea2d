use crate::display::text::Text;
/// Modul, ki predstavlja HUD (Heads-Up Display) v igri.
use crate::display::{
    button::{self, HealthBar},
    game_clock,
};
use crate::networking::shared::{SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::ttf;
use sdl2::video::{Window, WindowContext};

/// Struktura, ki predstavlja HUD v igri.
pub struct Hud<'a> {
    /// Seznam gumbov, ki so prikazani na HUD-u.
    pub buttons: Vec<button::Button<'a>>,
    /// Seznam značk (badges), ki so prikazane na HUD-u.
    pub badges: Vec<button::Badge>,
    /// Dropdown meni, ki je del HUD-a.
    pub dropdown: button::Dropdown<'a>,
    /// Prikazovalnik zdravja igralca.
    pub health_bar: button::HealthBar,
    /// Prikazovalnik časa igre.
    pub time_display: game_clock::GameClock,
}

impl<'a> Hud<'a> {
    /// Ustvari nov HUD z danimi gumbi, značkami, dropdown menijem in prikazovalnikom zdravja.
    pub fn new<'b: 'a>(
        gumbi: Vec<button::Button<'b>>,
        ikone: Vec<button::Badge>,
        meni: button::Dropdown<'b>,
        health: HealthBar,
    ) -> Hud<'b> {
        Hud {
            buttons: gumbi,
            badges: ikone,
            health_bar: health,
            dropdown: meni,
            time_display: game_clock::GameClock::new(),
        }
    }

    /// Izriše prikazovalnik časa na zaslonu.
    pub fn draw_time(
        &self,
        canvas: &mut Canvas<Window>,
        ttf_context: &ttf::Sdl2TtfContext,
        texture_creator: &TextureCreator<WindowContext>,
    ) {
        let time_text = self.time_display.formatted_time();
        let font_path = "resources/fonts/manolomono.otf";

        let font = ttf_context
            .load_font(font_path, 20)
            .expect("Failed to load font");

        let surface = font
            .render(&time_text)
            .blended(Color::RGB(0, 0, 0))
            .unwrap();

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let rect = Rect::new(
            10,
            (SCREEN_HEIGHT - 45) as i32,
            texture.query().width,
            texture.query().height,
        );
        canvas.copy(&texture, None, rect).unwrap();
    }

    /// Izriše HUD na zaslon.
    pub fn draw(
        &mut self,
        player_health: i32,
        level_index: i32,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        ttf_context: &sdl2::ttf::Sdl2TtfContext,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        texture_map: &mut std::collections::HashMap<String, Texture<'a>>,
    ) {
        canvas.set_draw_color(sdl2::pixels::Color::RGB(128, 128, 128));
        canvas
            .fill_rect(sdl2::rect::Rect::new(0, 0, SCREEN_WIDTH, 50))
            .unwrap();
        canvas
            .fill_rect(sdl2::rect::Rect::new(
                0,
                (SCREEN_HEIGHT - 50) as i32,
                SCREEN_WIDTH,
                50,
            ))
            .unwrap();

        // Izpiše nivo igre.
        let level_text = Text::new(
            650,
            15,
            20,
            "resources/fonts/manolomono.otf",
            format!("Level: {}", level_index),
            Color::RGB(255, 255, 255),
        );
        level_text.draw(canvas, ttf_context);

        // Izriše gumb HUD-a.
        for b in self.buttons.iter_mut() {
            b.draw(canvas, ttf_context, texture_creator, texture_map);
        }

        // Izriše značke HUD-a.
        for b in self.badges.iter_mut() {
            b.draw(canvas, texture_creator, texture_map);
        }

        // Izriše prikazovalnik časa.
        self.draw_time(canvas, ttf_context, texture_creator);

        // Izriše prikazovalnik zdravja igralca.
        self.health_bar.draw(player_health, canvas);

        // Izriše dropdown meni HUD-a.
        self.dropdown
            .draw(canvas, ttf_context, texture_creator, texture_map);
    }
}
