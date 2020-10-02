use crate::engine::*;

#[derive(Clone)]
pub struct Triangle<'a> {
    pub a: &'a Vec2,
    pub b: &'a Vec2,
    pub c: &'a Vec2,
}

impl Triangle<'_> {
    pub fn new<'a>(a: &'a Vec2, b: &'a Vec2, c: &'a Vec2) -> Triangle<'a> {
        Triangle { a, b, c }
    }
}

pub fn raster_triangle(a: &Vec2, b: &Vec2, c: &Vec2) -> Vec<((usize, usize), (f64, f64, f64))> {
    let mut points = vec![];
    let (x_min_f, x_max_f) = { get_min_and_max(a.x(), b.x(), c.x()) };
    let (y_min_f, y_max_f) = { get_min_and_max(a.y(), b.y(), c.y()) };

    let (x_min, y_min, x_max, y_max) = (
        x_min_f.floor() as usize,
        y_min_f.floor() as usize,
        x_max_f.ceil() as usize,
        y_max_f.ceil() as usize,
    );

    let mut has_entered_triangle_row = false;
    let mut has_entered_triangle_column = false;

    'row: for y in y_min..=y_max {
        'column: for x in x_min..=x_max {
            if let Some(bary_centric_params) = compute_bary_centric(x, y, a, b, c) {
                points.push(((x, y), bary_centric_params));
                has_entered_triangle_row = true;
                has_entered_triangle_column = true;
            } else if has_entered_triangle_column {
                //说明遇到三角形的像素之后, 第一次没有遇到三角形的像素, 那么这一行之后也不会有三角形的像素了.
                break 'column;
            }
        }
        if has_entered_triangle_row && !has_entered_triangle_column {
            //说明遇到三角形的像素之后, 整整一行都没有遇到三角形的像素了, 那么整个扫描后面也不会有三角形的像素了.
            break 'row;
        }
        has_entered_triangle_column = false;
        has_entered_triangle_row = false;
    }

    points
}

fn get_min_and_max<'a, T: PartialOrd + Copy>(a: T, b: T, c: T) -> (T, T) {
    let mut res = (a, a.clone());
    if b > res.1 {
        res.1 = b;
    }
    if b < res.0 {
        res.0 = b;
    }
    if c > res.1 {
        res.1 = c;
    }
    if c < res.0 {
        res.0 = c;
    }
    res
}

fn compute_bary_centric<'a>(
    x: usize,
    y: usize,
    p0: &Vec2,
    p1: &Vec2,
    p2: &Vec2,
) -> Option<(f64, f64, f64)> {
    let p = Vec2::new(x as f64, y as f64);
    let v0 = p1 - p0;
    let v1 = p2 - p0;
    let v2 = &p - p0;

    let dot_00 = Vec2::dot(&v0, &v0);
    let dot_01 = Vec2::dot(&v0, &v1);
    let dot_02 = Vec2::dot(&v0, &v2);
    let dot_11 = Vec2::dot(&v1, &v1);
    let dot_12 = Vec2::dot(&v1, &v2);

    let div = dot_00 * dot_11 - dot_01 * dot_01;
    if div == 0.0 {
        return None;
    }

    const EPSILON: f64 = -1e-9;

    let inv_divider = 1.0 / div;
    let alpha = (dot_11 * dot_02 - dot_01 * dot_12) * inv_divider;

    if alpha < EPSILON {
        return None;
    }

    let beta = (dot_00 * dot_12 - dot_01 * dot_02) * inv_divider;
    if beta < EPSILON {
        return None;
    }
    let gamma = 1.0 - alpha - beta;
    if gamma < EPSILON {
        return None;
    }

    Some((alpha, beta, gamma))
}

#[test]
fn test_raster_triangle() {
    let a = &Vec2::new(0.0, 30.0);
    let b = &Vec2::new(40.0, 0.0);
    let c = &Vec2::new(40.0, 30.0);
    let points = raster_triangle(a, b, c);

    assert_eq!(
        points.len(),
        641,
        "points should have area of 641, but got \n{}",
        crate::printer::render_points_to_string(points)
    );
}
