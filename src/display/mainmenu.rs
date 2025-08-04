use crate::display::button::{Button, ButtonAction};
use crate::display::text::Text;
use crate::environment::texture_data::TextureData;
use crate::game::GameState;
use crate::networking::shared::{SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::ttf;
use sdl2::video::Window;

pub struct MainMenu<'a> {
    pub start_button: Button<'a>,
    pub title: Text,
}

impl<'a> MainMenu<'a> {
    pub fn new<'b>(title_text: String) -> MainMenu<'b> {
        let dest_rect = Rect::new(
            (SCREEN_WIDTH / 2) as i32 - 75,
            (SCREEN_HEIGHT / 2) as i32 + 80,
            150,
            70,
        );
        let start_button = Button::new(
            ButtonAction::ChangeGameState(GameState::Running),
            Some("Start".to_string()),
            None,
            Color::RGB(109, 165, 194),
            dest_rect,
        );

        let title = Text::new(
            50,
            100,
            50,
            "resources/fonts/Battle-Race.ttf",
            title_text,
            Color::RGB(255, 255, 255),
        );
        MainMenu {
            start_button,
            title,
        }
    }

    // pub fn draw_menu_background(
    //     canvas: &mut Canvas<Window>,
    //     texture_creator: &TextureCreator<sdl2::video::WindowContext>,
    // ) -> Result<(), String> {
    //     let texture = texture_creator.load_texture("resources/textures/menu_background.png")?;

    //     // Assuming full-screen background
    //     let target = Rect::new(0, 0, 800, 600); // Replace with actual screen dimensions
    //     canvas.copy(&texture, None, Some(target))?;

    //     Ok(())
    // }

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
        let target = Rect::new(0, 0, 800, 600); // Replace with actual screen dimensions
        canvas.copy(&texture, None, Some(target)).unwrap();
        // canvas.set_draw_color(Color::RGB(44, 130, 201));
        // canvas.clear();
        // let rect = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
        // canvas.fill_rect(rect).unwrap();

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

        self.title.draw(canvas, ttf_context);
    }
}

pub fn init_mm<'a>(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ttf_context: &ttf::Sdl2TtfContext,
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    texture_map: &mut std::collections::HashMap<String, Texture<'a>>,
) {
    // Background
    canvas.set_draw_color(sdl2::pixels::Color::RGB(00, 00, 250));
    canvas.clear();
    let rect = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
    canvas.fill_rect(rect).unwrap();

    // Title -- add!!

    //Start button
    let dest_rect = Rect::new(
        (SCREEN_WIDTH / 2) as i32 - 75,
        (SCREEN_HEIGHT / 2) as i32 - 35,
        150,
        70,
    );
    let mut start_button = Button::new(
        ButtonAction::ChangeGameState(GameState::Running),
        Some("Start".to_string()),
        None,
        Color::RGB(0, 0, 0),
        dest_rect,
    );
    start_button.draw(canvas, ttf_context, texture_creator, texture_map);
}
