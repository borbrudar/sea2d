pub mod button;

use crate::hud::button::HealthBar;
use crate::networking::shared::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::wfc;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::render::{Canvas, Texture};
use sdl2::ttf;
use sdl2::video::Window;
use sdl2::video::WindowContext;

pub struct Hud<'a> {
    pub buttons: Vec<button::Button<'a>>,
    pub badges: Vec<button::Badge>,
    pub dropdown: button::Dropdown<'a>,
    pub health_bar: button::HealthBar,
    pub time_display: std::time::Instant,
}

impl<'a> Hud<'a> {
    pub fn new<'b: 'a>(
        gumbi: Vec<button::Button<'b>>,
        ikone: Vec<button::Badge>,
        meni: button::Dropdown<'b>,
        health: HealthBar,
        time: std::time::Instant,
    ) -> Hud<'b> {
        Hud {
            buttons: gumbi,
            badges: ikone,
            health_bar: health,
            dropdown: meni,
            time_display: time,
        }
    }

    pub fn draw_time(
        &self,
        canvas: &mut Canvas<Window>,
        ttf_context: &ttf::Sdl2TtfContext,
        texture_creator: &TextureCreator<WindowContext>,
    ) {
        let elapsed = self.time_display.elapsed();
        let seconds = elapsed.as_secs();
        let minutes = seconds / 60;
        let remaining_seconds = seconds % 60;

        let time_text = format!("{:02}:{:02}", minutes, remaining_seconds);
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

    pub fn draw(
        &mut self,
        player_health: i32,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        ttf_context: &sdl2::ttf::Sdl2TtfContext,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        texture_map: &mut std::collections::HashMap<String, Texture<'a>>,
    ) {
        // izrisi zadeve na ekranu, npr. health bar, score, etc.
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

        // narise gumbke
        for b in self.buttons.iter_mut() {
            b.draw(canvas, ttf_context, texture_creator, texture_map);
        }

        //narise badge
        for b in self.badges.iter_mut() {
            b.draw(canvas, texture_creator, texture_map);
        }

        // narise time
        self.draw_time(canvas, ttf_context, texture_creator);

        // narise health bar
        self.health_bar.draw(player_health, canvas);

        //narise ddm
        self.dropdown
            .draw(canvas, ttf_context, texture_creator, texture_map);

    }
}
