/// Modul za gumbe v igri.
use crate::environment::texture_data::TextureData;
use crate::game::GameState;
use crate::networking::shared::{SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::pixels::Color as RGB;
use sdl2::render::Texture;
use sdl2::ttf;
use sdl2::{event::Event, rect::Point, rect::Rect, render, video::WindowContext};

/// Gumb lahko sprejme dve vrsti dejanj: spreminja lahko stanje igre
/// ali pa izvede funkcijo.
pub enum ButtonAction<'a> {
    ChangeGameState(GameState),
    Callback(Box<dyn FnMut() + 'a>),
}

/// Struktura, ki predstavlja gumb v igri.
/// Gumb ima svoj ukaz, lahko vsebuje besedilo, teksturo in barvo
/// ter ima določen položaj na zaslonu.
pub struct Button<'a> {
    /// Ukaz.
    pub action: ButtonAction<'a>,
    /// Besedilo gumba.
    pub text: Option<String>,
    /// Tekstura gumba, če obstaja.
    pub texture: Option<TextureData>,
    /// Barva gumba.
    pub colour: Option<RGB>,
    /// Pravokotnik, ki predstavlja položaj gumba na zaslonu.
    pub position: Rect,
}

impl<'a> Button<'a> {
    /// Ustvari nov gumb z določenim dejanjem, besedilom, teksturo, barvo in položajem.
    pub fn new(
        act: ButtonAction<'a>,
        line: Option<String>,
        tex: Option<TextureData>,
        col: Option<RGB>,
        pos: Rect,
    ) -> Button<'a> {
        Button {
            action: act,
            text: line,
            texture: tex,
            colour: col,
            position: pos,
        }
    }

    /// Ustvari teksturo za besedilo gumba.
    /// Uporablja se za izris besedila na gumbu.
    pub fn create_text_texture<'b>(
        &'b self,
        texture_creator: &'b render::TextureCreator<WindowContext>,
        ttf_context: &'b ttf::Sdl2TtfContext,
    ) -> (render::Texture<'b>, u32, u32) {
        // Load a font
        let font_path = "resources/fonts/manolomono.otf";
        //let font = ttf_context.load_font(font_path, 24).unwrap();

        let font = ttf_context
            .load_font(font_path, 20)
            .expect("Failed to load font");

        let surface = font
            .render(self.text.as_ref().unwrap())
            .blended(RGB::RGB(255, 255, 255))
            .expect("Failed to create surface from font");

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .expect("Failed to create texture from surface");
        let render::TextureQuery { width, height, .. } = texture.query();

        (texture, width, height)
    }

    /// Izriše gumb na danem platnu.
    /// Gumb lahko vsebuje barvo, teksturo in besedilo.
    pub fn draw(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        ttf_context: &ttf::Sdl2TtfContext,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        texture_map: &mut std::collections::HashMap<String, Texture<'a>>,
    ) {
        if let Some(col) = self.colour {
            canvas.set_draw_color(col);
            canvas.fill_rect(self.position).unwrap();
        }
        //texture
        if let Some(tex_data) = self.texture.as_mut() {
            tex_data.load_texture(texture_creator, texture_map);
            tex_data
                .draw(
                    canvas,
                    texture_map,
                    self.position.x as f64,
                    self.position.y as f64,
                    self.position.width(),
                    self.position.height(),
                )
                .expect("Failed to draw texture");
        }
        //text
        if self.text.is_none() {
            return;
        }
        let ttc = canvas.texture_creator();
        let (texture, text_width, text_height) = self.create_text_texture(&ttc, ttf_context);
        let text_x = self.position.x + ((self.position.width() - text_width) / 2) as i32;
        let text_y = self.position.y + ((self.position.height() - text_height) / 2) as i32;

        let target = Rect::new(text_x, text_y, text_width, text_height);
        canvas.copy(&texture, None, Some(target)).unwrap();
    }

    /// Obravnava klike na gumb.
    pub fn handle_event(&mut self, event: &Event, game_state: &mut GameState) -> bool {
        if let Event::MouseButtonDown {
            timestamp: _,
            window_id: _,
            which: _,
            mouse_btn: _,
            clicks: _,
            x,
            y,
        } = event
        {
            if self.position.contains_point((*x, *y)) {
                match self.action {
                    ButtonAction::Callback(ref mut callback) => {
                        callback();
                    }
                    ButtonAction::ChangeGameState(state) => *game_state = state,
                }
                return true;
            }
        }
        false
    }
}

/// Struktura, ki predstavlja kazalec zdravja igralca.
pub struct HealthBar {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    pub _health: i32,
}

impl HealthBar {
    /// Ustvari nov kazalec zdravja z začetno širino, višino in položajem.
    pub fn new() -> HealthBar {
        HealthBar {
            width: 200,
            height: 30,
            x: (SCREEN_WIDTH - 205) as i32,
            y: (SCREEN_HEIGHT - 45) as i32,
            _health: 100,
        }
    }
    /// Izriše kazalec zdravja na danem platnu.
    /// Barva kazalca se spreminja glede na odstotek zdravja.
    pub fn draw(&self, health: i32, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let health_percent = health as f32 / 100.0;
        let fill_width = (self.width as f32 * health_percent - 2.) as i32;
        let fill_height = self.height - 2;
        canvas.set_draw_color(RGB::RGB(0, 0, 0));
        canvas
            .draw_rect(Rect::new(
                self.x,
                self.y,
                self.width as u32,
                self.height as u32,
            ))
            .unwrap();

        match health_percent {
            p if p > 0.8 => canvas.set_draw_color(RGB::RGB(0, 255, 0)),
            p if p > 0.5 => canvas.set_draw_color(RGB::RGB(255, 255, 0)),
            p if p > 0.2 => canvas.set_draw_color(RGB::RGB(255, 165, 0)),
            _ => canvas.set_draw_color(RGB::RGB(255, 0, 0)),
        }
        canvas
            .fill_rect(Rect::new(
                self.x + 1,
                self.y + 1,
                fill_width as u32,
                fill_height as u32,
            ))
            .unwrap();
    }
}

/// Struktura, ki predstavlja značko v igri.
pub struct Badge {
    pub position: Rect,
    pub texture: TextureData,
}

impl Badge {
    /// Ustvari novo značko z določenim položajem in teksturo.
    pub fn new(pos: Rect, tex: TextureData) -> Badge {
        Badge {
            position: pos,
            texture: tex,
        }
    }

    /// Izriše značko na danem platnu.
    pub fn draw<'a>(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        texture_map: &mut std::collections::HashMap<String, Texture<'a>>,
    ) {
        self.texture.load_texture(texture_creator, texture_map);
        self.texture
            .draw(
                canvas,
                texture_map,
                self.position.x as f64,
                self.position.y as f64,
                self.position.width(),
                self.position.height(),
            )
            .unwrap();
    }
}

//Struktura, ki predstavlja spustni meni (Dropdown menu).
pub struct Dropdown<'a> {
    /// Gumb, ki sproži prikaz spustnega menija.
    pub trigger: Button<'a>,
    /// Seznam gumbov, ki predstavljajo možnosti v spustnem meniju.
    pub items: Vec<Button<'a>>,
    /// Ali je spustni meni viden.
    pub visible: bool,
}

impl<'a> Dropdown<'a> {
    /// Ustvari nov spustni meni z danim sprožilnim gumbom in seznamom možnosti.
    pub fn new(trig: Button<'a>, stuff: Vec<Button<'a>>) -> Dropdown<'a> {
        Dropdown {
            trigger: trig,
            items: stuff,
            visible: false,
        }
    }

    /// Izriše spustni meni na danem platnu.
    pub fn draw(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        ttf_context: &ttf::Sdl2TtfContext,
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        texture_map: &mut std::collections::HashMap<String, Texture<'a>>,
    ) {
        self.trigger
            .draw(canvas, ttf_context, texture_creator, texture_map);
        if self.visible {
            for item in self.items.iter_mut() {
                item.draw(canvas, ttf_context, texture_creator, texture_map);
            }
        }
    }

    /// Obravnava dogodke miške v spustnem meniju.
    /// Preveri, ali je miška znotraj sprožilnega gumba ali katerega koli gumba v spustnem meniju.
    pub fn handle_event(&mut self, event: &Event) {
        if let Event::MouseMotion { x, y, .. } = event {
            let mouse_point = Point::new(*x, *y);
            let inside_trigger = self.trigger.position.contains_point(mouse_point);
            let inside_items = self
                .items
                .iter()
                .any(|item| item.position.contains_point(mouse_point));
            self.visible = inside_trigger || inside_items;
        }
    }
}
