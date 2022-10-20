use crate::graphics::colors::Color;
use crate::pixel_buf::{PixBufMutView, PixBufView};
use crate::shapes::{Line, Poly, Rect, Tri};
use crate::vectors::Vec2;

pub mod colors;
pub mod image;
pub mod tri_rasterizer;

pub struct Graphics<'a> {
    pub buffer: PixBufMutView<'a>,
    size: Vec2<u32>,
}
impl<'a> Graphics<'a> {
    pub fn new(buffer: PixBufMutView<'a>, size: Vec2<u32>) -> Self {
        Self { buffer, size }
    }

    #[inline(always)]
    pub fn size(&self) -> Vec2<u32> {
        self.size
    }

    pub fn draw_pixel(&mut self, pos: Vec2<i32>, color: Color) {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size.x as i32 || pos.y >= self.size.y as i32 {
            return;
        }
        // SAFETY: TODO
        unsafe {
            self.draw_pixel_unchecked(pos, color);
        }
    }
    #[inline(always)]
    pub unsafe fn draw_pixel_unchecked(&mut self, pos: Vec2<i32>, color: Color) {
        self.buffer.set_pixel_unchecked(pos, color); // TODO implement alpha blending
    }

    pub fn fill(&mut self, color: Color) {
        let buffer_size = self.buffer.bytes.len();

        let mut index = 0;
        while index < buffer_size {
            // SAFETY: TODO
            unsafe {
                self.buffer.set_pixel_by_index(index, color);
            }
            index += 4;
        }
    }

    pub fn draw_line(&mut self, line: &Line, color: Color) {
        let Line(mut from, to) = *line;

        let dist_x = (to.x - from.x).abs();
        let dist_y = (to.y - from.y).abs();

        let step_x = if from.x < to.x { 1 } else { -1 };
        let step_y = if from.y < to.y { 1 } else { -1 };

        let mut err = dist_x - dist_y;

        loop {
            self.draw_pixel(from, color);
            if from == to {
                break;
            }
            let e2 = err * 2;

            if e2 > -dist_y {
                err -= dist_y;
                from.x += step_x;
            }
            if e2 < dist_x {
                err += dist_x;
                from.y += step_y;
            }
        }
    }

    pub fn fill_col(&mut self, col: i32, mut from: i32, mut to: i32, color: Color) {
        let (self_w, self_h) = (self.size.x as i32, self.size.y as i32);

        // check that column is visible
        if col < 0 || col >= self_w || from >= self_w {
            return;
        }

        // clipping
        if from < 0 {
            from = 0
        }
        if to >= self_h {
            to = self_h - 1
        }

        // drawing
        for y in from..=to {
            self.draw_pixel(Vec2 { x: col, y }, color);
        }
    }
    pub fn fill_row(&mut self, row: i32, mut from: i32, mut to: i32, color: Color) {
        let (self_w, self_h) = (self.size.x as i32, self.size.y as i32);

        // check that row is visible
        if row < 0 || row >= self_h {
            return;
        }

        // clipping
        if from < 0 {
            from = 0
        }
        if from >= self_w {
            to = self_w - 1
        }

        // drawing
        for x in from..=to {
            self.draw_pixel(Vec2 { x, y: row }, color);
        }
    }

    pub fn draw_tri(&mut self, tri: &Tri, color: Color) {
        self.draw_line(&Line(tri.0, tri.1), color);
        self.draw_line(&Line(tri.1, tri.2), color);
        self.draw_line(&Line(tri.2, tri.0), color);
    }
    pub fn fill_tri(&mut self, tri: &Tri, color: Color) {
        tri_rasterizer::raster_tri(self, [tri.0, tri.1, tri.2], color);
    }

    pub fn draw_rect(&mut self, rect: &Rect, color: Color) {
        let l = rect.x;
        let t = rect.y;
        let r = l + rect.w;
        let b = t + rect.h;

        self.fill_col(l, t, b, color);
        self.fill_col(r, t, b, color);
        self.fill_row(t, l, r, color);
        self.fill_row(b, l, r, color);
    }
    pub fn fill_rect(&mut self, rect: &Rect, color: Color) {
        // clipping
        let self_w = self.size.x as i32;
        let self_h = self.size.y as i32;
        let [mut x, mut y, mut w, mut h]: [i32; 4] = rect.into();

        if x + w > self_w {
            w += self_w - (x + w)
        }
        if y + h >= self_h {
            h += self_h - (y + h)
        }
        if x < 0 {
            w += x;
            x = 0;
        }
        if y < 0 {
            h += y;
            y = 0;
        }

        // drawing
        for x in x..(x + w) {
            for y in y..(y + h) {
                self.draw_pixel(Vec2 { x, y }, color);
            }
        }
    }

    pub fn draw_poly(&mut self, poly: &Poly, color: Color) {
        if poly.points.len() < 2 {
            return;
        }

        for i in 1..poly.points.len() {
            self.draw_line(&Line(poly.points[i - 1], poly.points[i]), color);
        }
        self.draw_line(&Line(poly.points[0], *poly.points.last().unwrap()), color);
    }

    #[allow(unused_variables)]
    pub fn draw_str(&mut self, s: &str, color: Color) {
        todo!()
    }

    pub fn draw_pixels(&mut self, raster: PixBufView, rect: &Rect) {
        let Vec2 {
            x: raster_w,
            y: raster_h,
        } = raster.size;
        if rect.x == 0
            && rect.y == 0
            && rect.w == raster_w as i32
            && rect.h == raster_h as i32
            && raster_w == self.size.x
            && raster_h == self.size.y
        {
            self.draw_raster_1to1(raster);
            return;
        }

        self.shade_rect(rect, |pos| {
            let normal_x = (pos.x - rect.x) as f32 / (rect.w - 1) as f32;
            let img_x = (normal_x * (raster_w - 1) as f32) as i32;

            let normal_y = (pos.y - rect.y) as f32 / (rect.h - 1) as f32;
            let img_y = (normal_y * (raster_h - 1) as f32) as i32;

            // SAFETY: TODO
            unsafe { raster.get_pixel_unchecked(Vec2::new(img_x, img_y)) }
        });
    }
    pub fn draw_raster_1to1(&mut self, raster: PixBufView) {
        assert_eq!(raster.size, self.size);
        self.buffer.bytes.clone_from_slice(raster.bytes)
    }

    pub fn shade_rect(&mut self, rect: &Rect, color_fn: impl Fn(Vec2<i32>) -> Color) {
        // clipping
        let self_w = self.size.x as i32;
        let self_h = self.size.y as i32;
        let [mut x, mut y, mut w, mut h]: [i32; 4] = rect.into();

        if x + w >= self_w {
            w += self_w - (x + w)
        }
        if y + h >= self_h {
            h += self_h - (y + h)
        }
        if x < 0 {
            w += x;
            x = 0;
        }
        if y < 0 {
            h += y;
            y = 0;
        }
        let (x1, y1) = (x + w, y + h);

        // drawing
        for x in x..x1 {
            for y in y..y1 {
                // SAFETY: TODO
                unsafe {
                    self.draw_pixel_unchecked(Vec2::new(x, y), color_fn(Vec2::new(x, y)));
                }
            }
        }
    }

    pub fn fill_circle(&mut self, center: Vec2<i32>, radius: i32, color: Color) {
        let x0 = center.x - radius;
        let x1 = center.x + radius;
        let y0 = center.y - radius;
        let y1 = center.y + radius;
        let radius_sq = radius * radius;

        for x in x0..x1 {
            for y in y0..y1 {
                let line = Vec2::new(x, y) - center;
                let dist_sq = line.x * line.x + line.y * line.y;
                if dist_sq <= radius_sq {
                    self.draw_pixel(Vec2::new(x, y), color);
                }
            }
        }
    }
}
