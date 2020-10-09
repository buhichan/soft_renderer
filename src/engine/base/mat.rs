use super::vec4::Vec4;
use std::ops::{Add, Mul};

#[derive(Clone,Copy, Debug, PartialEq)]
pub struct Mat4 {
    pub value: [f64; 16],
}

impl std::fmt::Display for Mat4 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "\n")?;
        for i in 0..4 {
            write!(
                formatter,
                "|{},\t{},\t{},\t{}|\n",
                self.value[i * 4],
                self.value[i * 4 + 1],
                self.value[i * 4 + 2],
                self.value[i * 4 + 3]
            )?
        }
        Ok(())
    }
}

impl Mat4 {
    pub const IDENTITY: Mat4 = Mat4 {
        value: [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ],
    };
    pub fn new() -> Mat4 {
        Mat4 {
            value: [
                1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
            ],
        }
    }
}

macro_rules! mat4 {
    ($i: ident, $j: ident, $e: expr) => {{
        let mut res = [0.0; 16];
        for $i in 0..4 {
            for $j in 0..4 {
                res[$j * 4 + $i] = $e;
            }
        }
        Mat4 { value: res }
    }};
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4::new(
            self.value[0] * rhs.value[0]
                + self.value[1] * rhs.value[1]
                + self.value[2] * rhs.value[2]
                + self.value[3] * rhs.value[3],
            self.value[4] * rhs.value[0]
                + self.value[5] * rhs.value[1]
                + self.value[6] * rhs.value[2]
                + self.value[7] * rhs.value[3],
            self.value[8] * rhs.value[0]
                + self.value[9] * rhs.value[1]
                + self.value[10] * rhs.value[2]
                + self.value[11] * rhs.value[3],
            self.value[12] * rhs.value[0]
                + self.value[13] * rhs.value[1]
                + self.value[14] * rhs.value[2]
                + self.value[15] * rhs.value[3],
        )
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Mat4 {
        mat4!(
            i,
            j,
            self.value[0 + j * 4] * rhs.value[i]
                + self.value[1 + j * 4] * rhs.value[i + 4]
                + self.value[2 + j * 4] * rhs.value[i + 8]
                + self.value[3 + j * 4] * rhs.value[i + 12]
        )
    }
}

impl Mul<f64> for Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: f64) -> Mat4 {
        mat4!(i, j, self.value[j * 4 + i] * rhs)
    }
}

impl Add<Mat4> for Mat4 {
    type Output = Mat4;
    fn add(self, rhs: Mat4) -> Mat4 {
        mat4!(i, j, self.value[j * 4 + i] + rhs.value[j * 4 + i])
    }
}

#[test]
fn test_mat4_add() {
    let a = mat4!(i, j, (i * 2 + 3 * j) as f64);
    let b = mat4!(i, j, (i * 4 + 2 * j) as f64);

    let expect_a_plus_b = mat4!(i, j, (6 * i + 5 * j) as f64);

    let a_plus_b = a + b;

    assert_eq!(
        &a_plus_b, &expect_a_plus_b,
        "{} + {} yields {}, but should be {}",
        a, b, a_plus_b, expect_a_plus_b
    );
}

#[test]
fn test_mat4_mul() {
    let a = mat4!(i, j, (1 + i + 4 * j) as f64);
    let b = mat4!(i, j, (10 + i + 4 * j) as f64);

    let expect_a_mul_b = Mat4 {
        value: [
            180.0, 190.0, 200.0, 210.0, 
            436.0, 462.0, 488.0, 514.0, 
            692.0, 734.0, 776.0, 818.0,
            948.0, 1006.0, 1064.0, 1122.0,
        ] as [f64; 16],
    };

    let a_mul_b = a * b;

    assert_eq!(
        &a_mul_b, &expect_a_mul_b,
        "{} * {} yields {}, but should be {}",
        a, b, a_mul_b, expect_a_mul_b
    );
}
