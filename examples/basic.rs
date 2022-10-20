use rug::graphics::image::Image;
use rug::*;

struct App {
    img: Image,
    x: f32,
}
impl CanvasApp for App {
    fn setup(&mut self, _window: &mut Window) {
        //window.set_fullscreen(true);
    }

    fn render(&mut self, g: &mut Graphics, _window: &mut Window) {
        self.x += 1.0;

        //g.fill(rgba!(20, 20, 20));

        // let size = raster.size;
        g.draw_pixels(
            self.img.view(),
            &Rect::from_pos_size(Vec2::new(0, 0), g.size().map(|e| e as i32)),
        );

        g.fill_rect(&Rect::new(self.x as i32, 100, 50, 50), Color::RED);

        //g.fill_circle(v2!(220, 400), 200, Color::GREEN);
    }

    fn input_event(&mut self, event: InputEvent, window: &mut Window) {
        match event {
            InputEvent::KeyPressed(Key::F) => window.toggle_fullscreen(),
            InputEvent::KeyPressed(Key::Escape) => window.should_close = true,
            _ => {}
        }
    }
}

fn main() {
    let img = include_image!("img.png").unwrap();

    let app = App { x: 0.0, img };

    let config = CanvasConfig::new()
        .with_title("Basic Rug Test")
        .with_size(Vec2::new(600, 500));

    run_canvas_app(app, config);
}
