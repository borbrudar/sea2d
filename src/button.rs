// button nej sprejme neko callback funkcijo ki se bo izvedla ko kliknemo na gumb
// button bo imel tudi svoj text / sliko
// button bo imel svojo pozicijo in velikost
// probi dodat se dropdown menu

// aja life pro tip
// za uporabo drugih modulov v projektu uporabi use crate::modul::modul
// amapk mora bit dodan v main.rs kot `mod modul;` lhk pogledas

use crate::texture_data::TextureData;
use sdl2::pixels::Color as RGB;
use sdl2::ttf;
use sdl2::{event::Event, rect::Point, rect::Rect, render, video::WindowContext};

//basic button
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

impl<'a> Button<'a> {
    pub fn new(
        fun: Box<dyn FnMut() + 'a>,
        line: Option<String>,
        tex: Option<TextureData>,
        //texture_create: &'a render::TextureCreator<WindowContext>,
        col: RGB,
        pos: Rect,
    ) -> Button<'a> {
        Button {
            function: fun,
            text: line,
            texture: tex,
            //texture_creator: &texture_create,
            colour: col,
            position: pos,
        }
    }

    pub fn create_text_texture<'b>(
        &'b self,
        texture_creator: &'b render::TextureCreator<WindowContext>,
        ttf_context: &'b ttf::Sdl2TtfContext,
    ) -> (render::Texture<'b>, u32, u32) {
        // Load a font
        let font_path = "resources/fonts/Enwallowify-Regular.ttf";
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

    pub fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        ttf_context: &ttf::Sdl2TtfContext,
    ) {
        canvas.set_draw_color(self.colour);
        canvas.fill_rect(self.position).unwrap();

        //text
        let ttc = canvas.texture_creator();
        let (texture, text_width, text_height) = self.create_text_texture(&ttc, ttf_context);
        let text_x = self.position.x + ((self.position.width() - text_width) / 2) as i32;
        let text_y = self.position.y + ((self.position.height() - text_height) / 2) as i32;

        let target = Rect::new(text_x, text_y, text_width, text_height);
        canvas.copy(&texture, None, Some(target)).unwrap();
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

//health bar
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

//badges
pub struct Badge {
    pub position: Rect,
    pub texture: TextureData,
}

//dropdown menu
pub struct Dropdown<'a> {
    pub trigger: Button<'a>,
    pub items: Vec<Button<'a>>,
    pub visible: bool,
}

impl<'a> Dropdown<'a> {
    pub fn new(trig: Button<'a>, stuff: Vec<Button<'a>>) -> Dropdown<'a> {
        Dropdown {
            trigger: trig,
            items: stuff,
            visible: false,
        }
    }

    pub fn draw(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        ttf_context: &ttf::Sdl2TtfContext,
    ) {
        self.trigger.draw(canvas, ttf_context);
        if self.visible {
            for item in &self.items {
                item.draw(canvas, ttf_context);
            }
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::MouseMotion { x, y, .. } => {
                let mouse_point = Point::new(*x, *y);
                let inside_trigger = self.trigger.position.contains_point(mouse_point);
                let inside_items = self
                    .items
                    .iter()
                    .any(|item| item.position.contains_point(mouse_point));
                self.visible = inside_trigger || inside_items;
            }
            _ => {}
        }
    }
}
