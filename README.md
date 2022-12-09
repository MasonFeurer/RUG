## RUG - Rust Graphics Library
RUG is a Rust library for easily prototyping graphics using a canvas like API. 
RUG is not ideal for creating full-fledged apps because:
- RUG is not super efficient (it's not awfully slow though)
- RUG is maintained by me (this project will likely not be actively maintained)
- There are far better libraries ([bevy](https://crates.io/crates/bevy), [eframe](https://crates.io/crates/eframe), [tauri](https://crates.io/crates/tauri))

## Hello, World
To create an app, you must implement `CanvasApp` for some structure that would store the app's state. Then you can call `run_canvas_app` with an instance of the app and some window configurations.

NOTE: Ideally, the example would display "Hello, World!" on screen, but text graphics is not implemented yet.

```rs
use rug::*;

struct AppState;
impl CanvasApp for AppState {
    fn setup(&mut self, _window: &mut Window) {}

    fn render(&mut self, g: &mut Graphics, _window: &mut Window) {
        g.fill(Color::shade(220));
    }
}

fn main() {
    let config = CanvasConfig::new()
        .with_title("Hello, World!")
        .with_size(Vec2::new(400, 400));

    run_canvas_app(AppState, config);
}
```

