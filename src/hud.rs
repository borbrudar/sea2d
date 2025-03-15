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

pub struct Hud {}

impl Hud {
    pub fn new() -> Hud {
        Hud {}
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        // izrisi zadeve na ekranu, npr. health bar, score, etc.
        // spodnja koda ce odkomentiras izrise roza kvadrat na zacetku ekrana
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 255));
        canvas
            .fill_rect(sdl2::rect::Rect::new(0, 0, 100, 100))
            .unwrap();
    }
}
