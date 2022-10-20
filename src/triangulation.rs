use crate::shapes::Tri;
use crate::vectors::{Vec2, VecMath};

pub fn wrapped_index<T: Copy>(arr: &[T], index: i32) -> T {
    assert!(arr.len() < i32::MAX as usize);
    if index >= arr.len() as i32 {
        arr[(index % arr.len() as i32) as usize]
    } else if index < 0 {
        arr[(index % arr.len() as i32) as usize + arr.len()]
    } else {
        arr[index as usize]
    }
}

pub fn triangulate(vertices: &[Vec2<i32>]) -> Result<Vec<Tri>, String> {
    if vertices.len() < 3 {
        return Err("too few vertices, must have at-least 3".to_owned());
    }
    if i32::try_from(vertices.len()).is_err() {
        return Err("number of vertices must fit in an i32".to_owned());
    }
    // if !is_simple_poly(vertices) {
    // 	return Err("vertices do not define a simple poly".to_owned())
    // }
    // if contains_colinear_edges(vertices) {
    // 	return Err("vertices contains colinear edges".to_owned())
    // }

    // let (area, winding_order) = poly_area(vertices);

    // if winding_order == WindingOrder::CounterClockwise {
    // 	return Err("vertices must be in a clockwise winding order".to_owned())
    // }

    let mut index_list = Vec::with_capacity(vertices.len());
    for i in 0..vertices.len() {
        index_list.push(i);
    }

    let tri_count = vertices.len() - 2;

    let mut tris = Vec::with_capacity(tri_count);

    while index_list.len() > 3 {
        let mut found_ear = false;
        'find_ear: for i in 0..index_list.len() as i32 {
            let a_index = wrapped_index(&index_list, i);
            let b_index = wrapped_index(&index_list, i - 1);
            let c_index = wrapped_index(&index_list, i + 1);

            let a = vertices[a_index];
            let b = vertices[b_index];
            let c = vertices[c_index];

            let a_to_b = b - a;
            let a_to_c = c - a;

            if a_to_b.cross(a_to_c) > 0.0 {
                // reflex vertex
                continue;
            }

            for i in 0..vertices.len() {
                if i == a_index || i == b_index || i == c_index {
                    continue;
                }

                if Tri(b, a, c).contains_point(vertices[i as usize]) {
                    continue 'find_ear;
                }
            }

            tris.push((b_index, a_index, c_index));
            index_list.remove(i as usize);
            found_ear = true;
            break;
        }

        if !found_ear {
            return Err("failed to find ear".to_owned());
        }
    }
    tris.push((index_list[0], index_list[1], index_list[2]));
    let tris = tris
        .iter()
        .map(|e| Tri(vertices[e.0], vertices[e.1], vertices[e.2]))
        .collect();

    Ok(tris)
}

#[allow(unused_variables)]
pub fn is_simple_poly(vertices: &[Vec2<i32>]) -> bool {
    unimplemented!()
}

#[allow(unused_variables)]
pub fn contains_colinear_edges(vertices: &[Vec2<i32>]) -> bool {
    unimplemented!()
}

#[derive(Debug, Clone, PartialEq)]
pub enum WindingOrder {
    Clockwise,
    CounterClockwise,
}

#[allow(unused_variables)]
pub fn poly_area(vertices: &[Vec2<i32>]) -> (f32, WindingOrder) {
    unimplemented!()
}
