
pub fn rand2(i1: f64, i2: f64) -> f64{
    let t1 = i1 * 12.9783 + i2 * 78.323;
    return (t1.sin() * 43927.4582).fract();
}

pub fn rand3(i1: f64, i2: f64, i3: f64) -> f64 {
    let t1 = i1 * 12.9783 + i2 * 78.323 + i3 * 144.7373;
    return (t1.sin() * 43927.4582).fract();
}