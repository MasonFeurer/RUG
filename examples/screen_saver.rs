use rug::*;

struct AppState {
    img: Image,
    pos: Vec2<i32>,
    vel: Vec2<i32>,
}
impl CanvasApp for AppState {
    fn setup(&mut self, _window: &mut Window) {}

    fn render(&mut self, g: &mut Graphics, window: &mut Window) {
        g.fill(Color::BLACK);

        let size = Vec2::new(150, 150);

        g.draw_pixels(self.img.pixels(), &Rect::from_pos_size(self.pos, size));

        self.pos += self.vel;

        let win_size = window.size().map(|e| e as i32);

        if self.pos.x + size.x >= win_size.x {
            self.vel.x *= -1;
            self.pos.x = win_size.x - size.x;
        }
        if self.pos.x < 0 {
            self.vel.x *= -1;
            self.pos.x = 0;
        }

        if self.pos.y + size.y >= win_size.y {
            self.vel.y *= -1;
            self.pos.y = win_size.y - size.y;
        }
        if self.pos.y < 0 {
            self.vel.y *= -1;
            self.pos.y = 0;
        }
    }
}

fn main() {
    let app_state = AppState {
        img: include_image!("screen_saver_img.jpg").unwrap(),
        pos: Vec2::new(0, 0),
        vel: Vec2::new(3, 3),
    };
    let config = CanvasConfig::new()
        .with_title("Screen Saver")
        .with_size(Vec2::new(800, 600));
    run_canvas_app(app_state, config);
}
