use rug::*;

struct AppState;
impl CanvasApp for AppState {
    fn setup(&mut self, _window: &mut Window) {}

    fn render(&mut self, g: &mut Graphics, window: &mut Window) {
        g.fill(Color::RED);

        window.set_title(&format!("fps: {}", window.tracked_fps));
    }
}

fn main() {
    let config = CanvasConfig::new()
        .with_title("Hello, World!")
        .with_size(Vec2::new(400, 400));

    run_canvas_app(AppState, config);
}
