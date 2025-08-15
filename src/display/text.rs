use sdl2::pixels::Color;
use sdl2::render;
use sdl2::ttf;
use sdl2::{rect::Rect, video::WindowContext};

pub struct Text {
    x: i32,
    y: i32,
    pt: i32,
    font_path: &'static str,
    line: String,
    color: Color,
}

impl Text {
    pub fn new(
        x: i32,
        y: i32,
        pt: i32,
        font_path: &'static str,
        line: String,
        color: Color,
    ) -> Text {
        Text {
            x,
            y,
            pt,
            font_path,
            line,
            color,
        }
    }

    pub fn create_text_texture<'b>(
        &'b self,
        texture_creator: &'b render::TextureCreator<WindowContext>,
        ttf_context: &'b ttf::Sdl2TtfContext,
    ) -> (render::Texture<'b>, u32, u32) {
        // Load a font
        let font_path = self.font_path;

        let font = ttf_context
            .load_font(font_path, self.pt as u16)
            .expect("Failed to load font");

        let surface = font
            .render(self.line.as_str())
            .blended(Color::RGB(255, 255, 255))
            .expect("Failed to create surface from font");

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .expect("Failed to create texture from surface");
        let render::TextureQuery { width, height, .. } = texture.query();

        (texture, width, height)
    }

    pub fn draw(
        &self,
        canvas: &mut render::Canvas<sdl2::video::Window>,
        ttf_context: &ttf::Sdl2TtfContext,
    ) {
        let ttc = canvas.texture_creator();

        canvas.set_draw_color(self.color);
        let (texture, width, height) = self.create_text_texture(&ttc, ttf_context);
        let dest_rect = Rect::new(self.x, self.y, width, height);
        canvas.copy(&texture, None, Some(dest_rect)).unwrap();
    }
}
