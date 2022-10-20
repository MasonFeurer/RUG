use crate::graphics::{Color, Graphics};
use crate::vectors::Vec2;

// note: this rasterizer expects (0, 0) to be the top-left

/// Rasters a triangle represented by 3 points
pub fn raster_tri(g: &mut Graphics, mut points: [Vec2<i32>; 3], color: Color) {
    points.sort_by(|a, b| a.y.cmp(&b.y));
    raster_tri_sorted_y(g, points, color);
}
/// Rasters a triangle represented by 3 points, where the points are sorted by their y-coordinate's.
pub fn raster_tri_sorted_y(g: &mut Graphics, points: [Vec2<i32>; 3], color: Color) {
    let [top, mid, bot] = points;

    // if the middle's y matches the bottom's y, the bottom face is flat
    if mid.y == bot.y {
        raster_ffd_tri(g, points, color);
    }
    // if the middle's y matches the top's y, the top face is flat
    else if mid.y == top.y {
        raster_ffu_tri(g, points, color);
    } else {
        let other = Vec2 {
            x: (top.x as f32
                + ((mid.y - top.y) as f32 / (bot.y - top.y) as f32) * (bot.x - top.x) as f32)
                as i32,
            y: mid.y,
        };
        raster_ffd_tri(g, [top, mid, other], color);
        raster_ffu_tri(g, [mid, other, bot], color);
    }
}

/// Rasters a flat-face-down-triangle represented by 3 points
pub fn raster_ffd_tri(g: &mut Graphics, points: [Vec2<i32>; 3], color: Color) {
    let [top, b1, b2] = points;
    let r = if b1.x > b2.x { b1 } else { b2 };
    let l = if b1.x < b2.x { b1 } else { b2 };

    let xl = get_line_x(top, l);
    let xr = get_line_x(top, r);
    // xl & xr should be same len
    assert_eq!(xl.len(), xr.len());
    let len = xl.len();
    let mut y = top.y;

    for i in 0..len {
        g.fill_row(y, xl[i], xr[i], color);
        y += 1;
    }
}
/// Rasters a flat-face-up-triangle represented by 3 points
pub fn raster_ffu_tri(g: &mut Graphics, points: [Vec2<i32>; 3], color: Color) {
    let [t1, t2, bot] = points;
    let r = if t2.x > t1.x { t2 } else { t1 };
    let l = if t2.x < t1.x { t2 } else { t1 };

    let xl = get_line_x(bot, l);
    let xr = get_line_x(bot, r);
    // xl & xr should be same len
    assert_eq!(xl.len(), xr.len());
    let len = xl.len();
    let mut y = bot.y - 1;

    for i in 0..len {
        g.fill_row(y, xl[i], xr[i], color);
        y -= 1;
    }
}

pub fn get_line_x(mut from: Vec2<i32>, to: Vec2<i32>) -> Vec<i32> {
    let dist_x = (to.x - from.x).abs();
    let dist_y = (to.y - from.y).abs();

    let mut values = Vec::with_capacity(dist_y as usize);

    let step_x = if from.x < to.x { 1 } else { -1 };
    let step_y = if from.y < to.y { 1 } else { -1 };

    let mut err = dist_x - dist_y;

    loop {
        if from == to {
            return values;
        }
        let e2 = err * 2;

        if e2 > -dist_y {
            err -= dist_y;
            from.x += step_x;
        }
        if e2 < dist_x {
            err += dist_x;
            from.y += step_y;
            values.push(from.x);
        }
    }
}
