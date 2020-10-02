use crate::engine::base::Vec4;

pub fn raster_line(p0: &Vec4, p1: &Vec4) -> Vec<((usize, usize), f64)> {
    let (alpha, beta) = compute_line(p0, p1);

    let mut points = vec![];

    let start_point = (p0.x(), p0.y());
    let end_point = (p1.x(), p1.y());

    points.push((
        start_point.0.round() as usize,
        start_point.1.round() as usize,
    ));

    let mut cur_point = start_point.clone();

    let ((x_delta, x_delta_half), (y_delta, y_delta_half)) = (
        {
            if p1.x() > p0.x() {
                (1.0, 0.5)
            } else {
                (-1.0, -0.5)
            }
        },
        {
            if p1.y() > p0.y() {
                (1.0, 0.5)
            } else {
                (-1.0, -0.5)
            }
        },
    );

    let should_switch_x_and_y = alpha.abs() > 1.0;
    let mut mid_point = (0.0, 0.0);

    'l: loop {
        if should_switch_x_and_y {
            cur_point.1 += y_delta;
            if (cur_point.1 - end_point.1) * (cur_point.1 - start_point.1) > 0.0 {
                break 'l;
            }
            mid_point.0 = cur_point.0 + x_delta / 2.0;
            mid_point.1 = cur_point.1;
        } else {
            cur_point.0 += x_delta;
            if (cur_point.0 - end_point.0) * (cur_point.0 - start_point.0) > 0.0 {
                break 'l;
            }
            mid_point.0 = cur_point.0;
            mid_point.1 = cur_point.1 + y_delta / 2.0;
        }
        // 如果中点与下面的点在同一边, 则应该选择上面的点
        if (alpha * mid_point.0 + beta - mid_point.1) * (alpha * cur_point.0 + beta - cur_point.1)
            > 0.0
        {
            if should_switch_x_and_y {
                cur_point.0 += x_delta;
            } else {
                cur_point.1 += y_delta;
            }
        }
        points.push((cur_point.0.round() as usize, cur_point.1.round() as usize));
    }

    let unit_interpolate = 1.0 / (points.len() - 1) as f64;

    points
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i as f64 * unit_interpolate))
        .collect()
}

#[test]
fn test_raster_line() {
    const EPSILON: f64 = 1e-9;
    {
        let p0 = Vec4::new(2.0, 3.0, 0.0, 1.0);
        let p1 = Vec4::new(12.0, 10.0, 0.0, 1.0);

        let points = raster_line(&p0, &p1);

        let target: Vec<((usize, usize), f64)> = vec![
            ((2, 3), 0.0),
            ((3, 4), 0.1),
            ((4, 4), 0.2),
            ((5, 5), 0.3),
            ((6, 6), 0.4),
            ((7, 6), 0.5),
            ((8, 7), 0.6),
            ((9, 8), 0.7),
            ((10, 9), 0.8),
            ((11, 9), 0.9),
            ((12, 10), 1.0),
        ];

        assert!(
            points.iter().enumerate().all(|(i, x)| {
                let target_item = target.get(i).unwrap();
                x.0 == target_item.0 && x.1 - target_item.1 < EPSILON
            }),
            "case 1 failed with {:?}",
            points
        );
    }
    {
        let p0 = Vec4::new(12.0, 10.0, 0.0, 1.0);
        let p1 = Vec4::new(2.0, 3.0, 0.0, 1.0);

        let points = raster_line(&p0, &p1);
        let target: Vec<((usize, usize), f64)> = vec![
            ((12, 10), 0.0),
            ((11, 9), 0.1),
            ((10, 9), 0.2),
            ((9, 8), 0.3),
            ((8, 7), 0.4),
            ((7, 7), 0.5),
            ((6, 6), 0.6),
            ((5, 5), 0.7),
            ((4, 4), 0.8),
            ((3, 4), 0.9),
            ((2, 3), 1.0),
        ];

        assert!(
            points.iter().enumerate().all(|(i, x)| {
                let target_item = target.get(i).unwrap();
                x.0 == target_item.0 && x.1 - target_item.1 < EPSILON
            }),
            "case 2 failed with {:?}",
            points
        );
    }
    {
        let p0 = Vec4::new(3.0, 2.0, 0.0, 1.0);
        let p1 = Vec4::new(10.0, 12.0, 0.0, 1.0);

        let points = raster_line(&p0, &p1);

        let target: Vec<((usize, usize), f64)> = vec![
            ((3, 2), 0.0),
            ((4, 3), 0.1),
            ((4, 4), 0.2),
            ((5, 5), 0.3),
            ((6, 6), 0.4),
            ((6, 7), 0.5),
            ((7, 8), 0.6),
            ((8, 9), 0.7),
            ((9, 10), 0.8),
            ((9, 11), 0.9),
            ((10, 12), 1.0),
        ];
        assert!(
            points.iter().enumerate().all(|(i, x)| {
                let target_item = target.get(i).unwrap();
                x.0 == target_item.0 && x.1 - target_item.1 < EPSILON
            }),
            "case 3 failed with {:?}",
            points
        );
    }
}

fn compute_line(p0: &Vec4, p1: &Vec4) -> (f64, f64) {
    // alpha * p0.x() + beta === p0.y()
    // alpha * p1.x() + beta === p1.y()
    // alpha * p0.x() * p1.x() + beta * p1.x() === p0.y() * p1.x()
    // alpha * p0.x() * p1.x() + beta * p0.x() === p1.y() * p0.x()
    let alpha = (p1.y() - p0.y()) / (p1.x() - p0.x());
    let beta = (p1.y() * p0.x() - p0.y() * p1.x()) / (p0.x() - p1.x());

    (alpha, beta)
}

#[test]
fn test_compute_line() {
    let p0 = Vec4::new(12.0, 32.0, 0.0, 1.0);
    let p1 = Vec4::new(233.0, 31.0, 0.0, 1.0);

    let (alpha, beta) = compute_line(&p0, &p1);

    const EPSILON: f64 = 1e-9;

    assert!(
        (alpha * p0.x() + beta - p0.y()).abs() < EPSILON,
        "{} * {} + {} = {} but should be {}",
        alpha,
        p0.x(),
        beta,
        alpha * p0.x() + beta,
        p0.y()
    );
    assert!(
        (alpha * p1.x() + beta - p1.y()).abs() < EPSILON,
        "{} * {} + {} = {} but should be {}",
        alpha,
        p1.x(),
        beta,
        alpha * p1.x() + beta,
        p1.y()
    );
}
