// NALOGA: implementiraj heads up display - HUD
// torej kar hoces da je na enkranu
// draw rab usaj self pa ta canvas k rise zadeve, to je en razred iz sdl2 poglej mal dokumentaicjo za uporabo tega
// na netu
// alpa kle kodo mislm da v tile.rs risem neke kvadrate sicer tm iz texture ampk ja
// ah sej res u aabb.rs je funkcija draw za prou kvadrate risat ampk to je cist na meji

// ce hoces da so neke texture uporab TextureData struct
// k mam nek HashMap u client.rs ki poskrbi da se usaka textura zloada samo enkrat in mam pol cachanu
// tk da ce jo rabm jo od tm klicem
// zato buls da uporabs to stvar

// aja pa ne pozabi delat na dev branchu in ne na main drgac ti bom revertov k bodo merge conflicti :X
// ce bodo pol nakonc mergi bom js urejou sm naceloma nej nebi bli
// sm ne spreminjat drugih fileov kej prevec, raj dodej nove ce kej ne ves, bi mogl bit integriran

// sej bom naceloma probu kodo u kratkem mal bolj komentirat ampk kr upras ce ti ni jasn
// pa uso sreco z implementacijo ;D
use crate::{
    button,
    shared::{SCREEN_HEIGHT, SCREEN_WIDTH},
};

pub struct Hud<'a> {
    pub buttons: Vec<button::Button<'a>>,
    pub badges: Vec<button::Badge>,
    pub dropdown: button::Dropdown<'a>,
    pub health_bar: button::HealthBar,
    pub time_display: u32,
}

impl<'a> Hud<'a> {
    pub fn new<'b: 'a>(gumbi: Vec<button::Button<'b>>, meni: button::Dropdown<'b>) -> Hud<'b> {
        Hud {
            buttons: gumbi, //pause button hočem, da je po defaultu na vsakem levelu plus others
            badges: Vec::new(),
            health_bar: button::HealthBar::new(100, 100, 200, 20),
            dropdown: meni,
            time_display: 0,
        }
    }

    pub fn draw(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
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
        for b in self.buttons.iter() {
            b.draw(canvas);
        }

        //narise ddm
        self.dropdown.draw(canvas);
    }
}
