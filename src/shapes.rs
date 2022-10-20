use crate::vectors::{Vec2, VecMath};

#[derive(Clone, PartialEq, Debug)]
pub struct Poly {
    /// points of the polygon, stored in a clock-wise winding order order
    pub points: Vec<Vec2<i32>>,
}
impl Poly {
    pub fn empty() -> Self {
        Self { points: Vec::new() }
    }
    pub fn new(points: &[Vec2<i32>]) -> Self {
        Self {
            points: points.to_vec(),
        }
    }

    pub fn contains_point(&self, _p: Vec2<i32>) -> bool {
        unimplemented!()
    }
    pub fn lines(&self) -> Vec<Line> {
        let len = self.points.len();
        let mut lines = Vec::with_capacity(len - 1);
        for i in 1..len {
            lines.push(Line(self.points[i - 1], self.points[i]));
        }
        lines.push(Line(self.points[len - 1], self.points[0]));
        lines
    }
}

// points should be stored in a clock-wise winding order
#[derive(Clone, PartialEq, Debug)]
pub struct Tri(pub Vec2<i32>, pub Vec2<i32>, pub Vec2<i32>);
impl Tri {
    pub const fn new(verts: &[Vec2<i32>; 3]) -> Self {
        Self(verts[0], verts[1], verts[2])
    }

    pub const fn lines(&self) -> [Line; 3] {
        [
            Line(self.0, self.1),
            Line(self.1, self.2),
            Line(self.2, self.0),
        ]
    }

    // this method expects the points in this tri to be in a clockwise-winding order
    pub fn contains_point(&self, p: Vec2<i32>) -> bool {
        let a_to_b = self.1 - self.0;
        let b_to_c = self.2 - self.1;
        let c_to_a = self.0 - self.2;

        let a_to_p = p - self.0;
        let b_to_p = p - self.1;
        let c_to_p = p - self.2;

        let cross1 = a_to_b.cross(a_to_p);
        let cross2 = b_to_c.cross(b_to_p);
        let cross3 = c_to_a.cross(c_to_p);

        if cross1 < 0.0 || cross2 < 0.0 || cross3 < 0.0 {
            false
        } else {
            true
        }
    }
}
impl From<&Tri> for [Vec2<i32>; 3] {
    fn from(tri: &Tri) -> Self {
        [tri.0, tri.1, tri.2]
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Line(pub Vec2<i32>, pub Vec2<i32>);
impl Line {
    pub fn intersects_line(&self, other: &Line) -> bool {
        lines_intersect(self, other)
    }

    // idk if this works :/ (havnt tested it)
    pub fn line_intersection(&self, other: &Line) -> Option<Vec2<i32>> {
        let xdiff = Vec2::new(self.0.x - self.1.x, other.0.x - other.1.x);
        let ydiff = Vec2::new(self.0.y - self.1.y, other.0.y - other.1.y);

        fn det(a: Vec2<i32>, b: Vec2<i32>) -> i32 {
            a.x * b.y - a.y * b.x
        }

        let div = det(xdiff, ydiff);
        if div == 0 {
            return None;
        }
        let d = Vec2::new(det(self.0, self.1), det(other.0, other.1));
        Some(Vec2::new(det(d, xdiff), det(d, ydiff)))
    }

    #[inline(always)]
    pub fn min_x(&self) -> i32 {
        std::cmp::min(self.0.x, self.1.x)
    }
    #[inline(always)]
    pub fn max_x(&self) -> i32 {
        std::cmp::max(self.0.x, self.1.x)
    }
    #[inline(always)]
    pub fn min_y(&self) -> i32 {
        std::cmp::min(self.0.y, self.1.y)
    }
    #[inline(always)]
    pub fn max_y(&self) -> i32 {
        std::cmp::max(self.0.y, self.1.y)
    }
}
impl From<&Line> for [Vec2<i32>; 2] {
    fn from(line: &Line) -> Self {
        [line.0, line.1]
    }
}

// idk if this works :/ (havnt tested it)
pub fn lines_intersect(a: &Line, b: &Line) -> bool {
    let (p, q, r, s) = (b.0.x, b.0.y, b.1.x, b.1.y);
    let (a, b, c, d) = (a.0.x, a.0.y, a.1.x, a.1.y);

    let det: i32 = (c - a) * (s - q) - (r - p) * (d - b);
    if det == 0 {
        false
    } else {
        let lambda = ((s - q) * (r - a) + (p - r) * (s - b)) as f32 / det as f32;
        let gamma = ((b - d) * (r - a) + (c - a) * (s - b)) as f32 / det as f32;
        (0.0 < lambda && lambda < 1.0) && (0.0 < gamma && gamma < 1.0)
    }
}

// TODO read tutuorial on how this thing works :>
pub fn project_point_onto_line(p: Vec2<i32>, line: &Line) -> Vec2<i32> {
    let Line(v1, v2) = *line;

    // get dot product of e1, e2
    let e1: Vec2<i32> = Vec2::new(v2.x - v1.x, v2.y - v1.y);
    let e2: Vec2<i32> = Vec2::new(p.x - v1.x, p.y - v1.y);
    let val_dp: f64 = e1.dot(e2);

    // get squared length of e1
    let len2: f64 = e1.len_sq();

    let result_x = (v1.x as f64 + (val_dp * e1.x as f64) / len2) as i32;
    let result_y = (v1.y as f64 + (val_dp * e1.y as f64) / len2) as i32;
    Vec2::new(result_x, result_y)
}
pub fn line_contains_point(line: &Line, width: i32, point: Vec2<i32>) -> bool {
    let max_dist_sq = (width as f64 * 0.5) * (width as f64 * 0.5);

    let projected = project_point_onto_line(point, line);

    let dist_sq = (projected - point).abs_len_sq();

    dist_sq <= max_dist_sq
        && point.x >= line.min_x()
        && point.x <= line.max_x()
        && point.y >= line.min_y()
        && point.y <= line.max_y()
}

#[derive(Clone, PartialEq, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}
impl Rect {
    pub const fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }
    pub const fn from_pos_size(pos: Vec2<i32>, size: Vec2<i32>) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            w: size.x,
            h: size.y,
        }
    }

    #[inline(always)]
    pub const fn pos(&self) -> Vec2<i32> {
        Vec2::new(self.x, self.y)
    }
    #[inline(always)]
    pub const fn size(&self) -> Vec2<i32> {
        Vec2::new(self.w, self.h)
    }
    #[inline(always)]
    pub const fn tl(&self) -> Vec2<i32> {
        Vec2::new(self.x, self.y)
    }
    #[inline(always)]
    pub const fn tr(&self) -> Vec2<i32> {
        Vec2::new(self.x + self.w, self.y)
    }
    #[inline(always)]
    pub const fn br(&self) -> Vec2<i32> {
        Vec2::new(self.x + self.w, self.y + self.h)
    }
    #[inline(always)]
    pub const fn bl(&self) -> Vec2<i32> {
        Vec2::new(self.x, self.y + self.h)
    }

    pub const fn contains_point(&self, p: Vec2<i32>) -> bool {
        p.x >= self.x && p.x <= self.x + self.w && p.y >= self.y && p.y <= self.y + self.h
    }
    pub const fn points(&self) -> [Vec2<i32>; 4] {
        [self.tl(), self.tr(), self.br(), self.bl()]
    }
    // TL to TR, TR to BR, BR to BL, BL to TL
    pub const fn lines(&self) -> [Line; 4] {
        [
            Line(self.tl(), self.tr()),
            Line(self.tr(), self.br()),
            Line(self.br(), self.bl()),
            Line(self.bl(), self.tl()),
        ]
    }
}
impl From<&Rect> for [i32; 4] {
    fn from(rect: &Rect) -> Self {
        [rect.x, rect.y, rect.w, rect.h]
    }
}

pub fn remove_dup_points<T: PartialEq>(points: &mut Vec<Vec2<T>>) {
    for i in (0..points.len()).rev() {
        let mut dup = None;
        for j in 0..i {
            if points[j] == points[i] {
                dup = Some(i);
                break;
            }
        }
        if let Some(dup) = dup {
            points.remove(dup);
            continue;
        }
        for j in i + 1..points.len() {
            if points[j] == points[i] {
                dup = Some(i);
                break;
            }
        }
        if let Some(dup) = dup {
            points.remove(dup);
        }
    }
}
