// button nej sprejme neko callback funkcijo ki se bo izvedla ko kliknemo na gumb
// button bo imel tudi svoj text / sliko
// button bo imel svojo pozicijo in velikost
// probi dodat se dropdown menu

// aja life pro tip
// za uporabo drugih modulov v projektu uporabi use crate::modul::modul
// amapk mora bit dodan v main.rs kot `mod modul;` lhk pogledas

use crate::shared::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::texture_data::TextureData;
//use sdl2::gfx::primitives::DrawRenderer;
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
    offset: i32,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    pub health: i32,
}

impl HealthBar {
    pub fn new() -> HealthBar {
        HealthBar {
            offset: 20,
            width: 200,
            height: 30,
            x: (SCREEN_WIDTH - 205) as i32,
            y: (SCREEN_HEIGHT - 45) as i32,
            health: 100,
        }
    }

    pub fn draw(&self, health: i32, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let health_percent = health as f32 / 100.0;
        let fill_width = (self.width as f32 * health_percent) as i32;
        let fill_height = self.height;
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
                self.x,
                self.y,
                fill_width as u32,
                fill_height as u32,
            ))
            .unwrap();

        /*
        // Points of the parallelogram
        let p0 = (self.x, self.y);
        let p1 = (self.x + self.width, self.y);
        let p2 = (self.x + self.width - self.offset, self.y + self.height);
        let p3 = (self.x - self.offset, self.y + self.height);

        // Health percent (from 0.0 to 1.0)
        let health_percent = self.health as f32 / 100.0;

        // Interpolate points for filled area
        let fill_p1_x = self.x + (self.width as f32 * health_percent) as i32;
        let fill_p2_x = self.x + (self.width as f32 * health_percent) as i32 - self.offset;

        let fill_points = [p0, (fill_p1_x, p1.1), (fill_p2_x, p2.1), p3];

        let vx = [p0.0, p1.0, p2.0, p3.0];
        let vy = [p0.1, p1.1, p2.1, p3.1];

        canvas
            .filled_polygon(
                &vx.iter().map(|&x| x as i16).collect::<Vec<i16>>(),
                &vy.iter().map(|&y| y as i16).collect::<Vec<i16>>(),
                RGB::RGB(50, 50, 50),
            )
            .unwrap();

        let vxf = [
            fill_points[0].0,
            fill_points[1].0,
            fill_points[2].0,
            fill_points[3].0,
        ];
        let vyf = [
            fill_points[0].1,
            fill_points[1].1,
            fill_points[2].1,
            fill_points[3].1,
        ];

        canvas
            .filled_polygon(
                &vxf.iter().map(|&x| x as i16).collect::<Vec<i16>>(),
                &vyf.iter().map(|&y| y as i16).collect::<Vec<i16>>(),
                RGB::RGB(255, 0, 0),
            )
            .unwrap();
        */
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
