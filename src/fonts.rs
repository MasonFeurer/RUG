use crate::shapes::{remove_dup_points, Poly};
use crate::vectors::Vec2;
pub use rusttype::Font;
use rusttype::{OutlineBuilder, Point, Scale};

#[macro_export]
macro_rules! include_font {
    ($path:literal) => {{
        $crate::fonts::Font::try_from_bytes(include_bytes!($path))
    }};
}

pub fn build_text(text: &str, pos: Vec2<i32>, font: &Font, size: f32) -> Vec<Poly> {
    let layout = font.layout(
        text,
        Scale::uniform(size),
        Point {
            x: pos.x as f32,
            y: pos.y as f32,
        },
    );
    let mut builder = TextBuilder::new();
    for glyph in layout {
        let pos = glyph.position();
        builder.glyph_pos = Vec2::new(pos.x as i32, pos.y as i32);
        glyph.build_outline(&mut builder);
    }
    builder.polys.pop();
    for poly in &mut builder.polys {
        remove_dup_points(&mut poly.points);
    }
    builder.polys
}

pub struct TextBuilder {
    // on init, should have 1 blank poly pushed
    pub polys: Vec<Poly>,
    pub pos: Vec2<i32>,
    pub glyph_pos: Vec2<i32>,
}
impl TextBuilder {
    pub fn new() -> Self {
        let mut polys = Vec::new();
        polys.push(Poly::empty());
        Self {
            polys,
            pos: Vec2::new(0, 0),
            glyph_pos: Vec2::new(0, 0),
        }
    }

    pub fn vertex(&mut self, pos: Vec2<i32>) {
        let x = pos.x + self.glyph_pos.x;
        let y = pos.y + self.glyph_pos.y;

        self.polys.last_mut().unwrap().points.push(Vec2::new(x, y));
    }
}
impl OutlineBuilder for TextBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.pos = Vec2::new(x as i32, y as i32);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.vertex(Vec2::new(x as i32, y as i32));
        self.pos = Vec2::new(x as i32, y as i32);
    }

    // (x1,y1) is the control
    fn quad_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let a = self.pos;
        let c = Vec2::new(x1 as i32, y1 as i32);
        let b = Vec2::new(x2 as i32, y2 as i32);

        fn lerp(a: Vec2<i32>, b: Vec2<i32>, t: f32) -> Vec2<i32> {
            // `t` should be normalized
            let dist_x = ((b.x as f32 - a.x as f32) * t) as i32;
            let dist_y = ((b.y as f32 - a.y as f32) * t) as i32;
            Vec2::new(a.x + dist_x, a.y + dist_y)
        }

        let mut t = 0.0;
        while t <= 1.0 {
            let lerp_ac = lerp(a, c, t);
            let lerp_bc = lerp(c, b, t);

            self.vertex(lerp(lerp_ac, lerp_bc, t));

            t += 0.5;
        }
        self.pos = b;
    }

    fn curve_to(&mut self, _x1: f32, _y1: f32, _x2: f32, _y2: f32, _x: f32, _y: f32) {
        // TODO (cubic bezier curves)
        unimplemented!("Currently only supports TrueType fonts")
    }

    // when a vertex loop in the font has closed
    fn close(&mut self) {
        // push blank poly into vec
        self.polys.push(Poly::empty());
    }
}
