pub mod fonts;
pub mod graphics;
pub mod input;
pub mod pixel_buf;
pub mod shapes;
pub mod tri_rasterizer;
pub mod triangulation;
pub mod vectors;
pub mod window;

pub use graphics::{Color, Graphics, Image};
pub use input::{InputEvent, Key, MouseButton};
pub use shapes::{Line, Poly, Rect, Tri};
pub use vectors::{Vec2, Vec3, Vec4, VecMath};
pub use window::{run_canvas_app, CanvasApp, CanvasConfig, Window};

#[test]
fn test() {
    println!("lib compiles");
}
