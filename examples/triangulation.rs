use rug::fonts::build_text;
use rug::triangulation::triangulate;
use rug::*;

pub struct App {
    polys: Vec<Poly>,
    tris: Vec<Tri>,
}
impl App {
    fn new() -> Self {
        let font = include_font!("menlo-regular.ttf").unwrap();

        #[cfg(target_os = "macos")]
        let size = 1200.0;
        #[cfg(target_os = "windows")]
        let size = 600.0;

        let polys = build_text("A", Vec2::new(40, 80), &font, size);

        let tris = triangulate(&polys[1].points).unwrap();

        Self { polys, tris }
    }
}
impl CanvasApp for App {
    fn setup(&mut self, _window: &mut Window) {}

    fn render(&mut self, g: &mut Graphics, _window: &mut Window) {
        g.fill(Color::BLACK);

        for poly in &self.polys {
            g.draw_poly(poly, Color::WHITE);
        }

        for tri in &self.tris {
            g.draw_tri(tri, Color::GREEN);
        }
    }
}

fn main() {
    #[cfg(target_os = "macos")]
    let size = Vec2::new(1200, 1200);
    #[cfg(target_os = "windows")]
    let size = Vec2::new(600, 600);

    let config = CanvasConfig::new()
        .with_title("Poly Triangulation")
        .with_size(size);

    run_canvas_app(App::new(), config)
}
