
use crate::engine::Frame;
extern crate colored;
use colored::Colorize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Write;
use std::fs::write;

pub fn render_frame_to_stdout(frame: &Frame) {
    for y in 0..frame.height {
        for x in 0..frame.width  {
            if let Some(pixel) = frame.get(&(x, y)) {
                let color = pixel.color;
                print!("{}",&"▇".truecolor(color.0, color.1, color.2).to_string())
            }else{
                print!(" ")
            }
        }
        print!("|\n");
    }

}

pub fn render_points_to_string(points: Vec<((usize, usize), (f64, f64, f64))>) -> String {
    let max = points.iter().fold((0, 0), |mut res, cur| {
        if (cur.0).0 > res.0 {
            res.0 = (cur.0).0
        }
        if (cur.0).1 > res.1 {
            res.1 = (cur.0).1
        }
        res
    });
    let mut res = "".to_string();
    let hash_set: HashMap<(usize, usize), ((usize, usize), (f64, f64, f64))> =
        points.into_iter().map(|x| (x.0, x)).collect();
    for y in 0..=max.1 {
        for x in 0..=max.0 {
            if hash_set.contains_key(&(x, y)) {
                res += "x";
            } else {
                res += " "
            }
        }
        res += "|\n"
    }
    res
}

pub fn render_to_ppm(frame: &Frame) -> String {
    let mut res = format!("P3\n{} {}\n255\n",frame.width,frame.height).to_string();
    for y in 0..frame.height {
        for x in 0..frame.width {
            if let Some(pixel) = frame.get(&(x, y)) {
                res += format!("{} {} {}\n", (pixel.color.0), pixel.color.1, pixel.color.2 ).as_str();
            } else {
                res += "0 0 0\n"
            }
        }
    }
    res
}