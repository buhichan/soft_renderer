use crate::engine::*;
use crate::object::*;

pub struct Camera {
    pub projection_matrix: Mat4,
    pub position: Vec4,
    pub rotation: Mat4,
    pub view_matrix: Mat4,

    /**
     * fov, in radians
     */
    pub fov: f64,
    pub aspect_ratio: f64,

    pub near: f64,
    pub far: f64,
}

impl Camera {
    pub fn new(up:Vec4, front:Vec4, position:Vec4) -> Camera {
        let rotation = Mat4::IDENTITY.clone();

        let mut new_camera = Camera {
            projection_matrix: Mat4::IDENTITY,
            position,
            rotation,
            view_matrix: Mat4::IDENTITY,
            near: 0.1,
            far: 100.,
            fov: std::f64::consts::PI / 4.0,
            aspect_ratio: 3.0 / 4.0,
        };

        new_camera.set_up(&up);
        new_camera.set_front(&front);
        new_camera.recompute_projection_matrix();
        new_camera.recompute_view_matrix();

        new_camera
    }
    pub fn recompute_view_matrix(&mut self) {

        let right = Vec4::cross(self.get_front(), self.get_up());

        for i in 0..3 {
            self.rotation.value[4 * i] = right.value[i];
        }

        let position_matrix = Mat4 {
            value: [
                1.0, 0.0, 0.0, -self.position.x(),
                0.0, 1.0, 0.0, -self.position.y(),
                0.0, 0.0, 1.0, -self.position.z(),
                0.0, 0.0, 0.0, 1.0,
            ],
        };

        self.view_matrix = self.rotation * position_matrix;
    }
    /**
     * borrowed from https://github.com/mrdoob/three.js/blob/dev/src/math/Matrix4.js
     */
    pub fn recompute_projection_matrix(&mut self) {
        let te = &mut self.projection_matrix.value;
        let Camera {
            near,
            far,
            fov,
            aspect_ratio,
            ..
        } = *self;
        let tan_theta_2 = (fov / 2.0).tan();
        let width = 2.0 * near * tan_theta_2;
        let height = width * aspect_ratio;
        let x = 2.0 * near / width;
        let y = 2.0 * near / height;

        let c = -(far + near) / (far - near);
        let d = -2.0 * far * near / (far - near);

        te[0] = x;
        te[4] = 0.;
        te[8] = 0.0;
        te[12] = 0.;
        te[1] = 0.;
        te[5] = y;
        te[9] = 0.0;
        te[13] = 0.;
        te[2] = 0.;
        te[6] = 0.;
        te[10] = c;
        te[14] = d;
        te[3] = 0.;
        te[7] = 0.;
        te[11] = -1.;
        te[15] = 0.;
    }
}

impl Object for Camera {
    fn set_position(&mut self, pos: &Vec4) {
        self.position = pos.clone();
    }
    fn get_position(&self) -> &Vec4 {
        &self.position
    }
    fn get_front(&self) -> Vec4 {
        Vec4::new(
            -self.view_matrix.value[2],
            -self.view_matrix.value[6],
            -self.view_matrix.value[10],
            1.0,
        )
    }
    fn set_front(&mut self, front: &Vec4) {
        for i in 0..3 {
            self.rotation.value[4 * i + 2] = -front.value[i];
        }
    }
    fn get_up(&self) -> Vec4 {
        Vec4::new(
            self.view_matrix.value[1],
            self.view_matrix.value[5],
            self.view_matrix.value[9],
            1.0,
        )
    }
    fn set_up(&mut self, up: &Vec4) {
        for i in 0..3 {
            self.rotation.value[4 * i + 1] = up.value[i];
        }
    }
}
