use super::raster::*;
use crate::engine::base::*;
use crate::engine::frame::*;
use crate::engine::program::{Program, ShaderData};

pub struct Context {
    pub near: f64,
    pub far: f64,
    pub current_program: Program,
    pub current_buffers: Vec<Vec<Vec4>>,
    pub current_frame: Frame,
}

#[derive(Debug)]
struct Vertex {
    pub coord: Vec2,
    pub index: usize,
    pub depth: f64,
}

impl<'a> Context{
    fn default_color() -> Vec4 {
        Vec4::new(0.0, 0.0, 0.0, 1.0)
    }
    pub fn draw_triangles(&'a mut self, vertex_buffer_index: usize) {
        let vertices = self
            .current_buffers
            .get(vertex_buffer_index)
            .unwrap()
            .clone();
        self.projection(vertices)
    }
    fn projection(&'a mut self, vertices: Vec<Vec4>) {
        let current_uniform_values = self
            .current_program
            .uniforms
            .iter()
            .map(|unif| unif.value.clone())
            .collect();

        let mut total_vertices: Vec<Vertex> = vec![];
        // let mut i = 0;
        let mut current_attribute_values: Vec<Vec<ShaderData>> = vec![];

        for (vertex_index, vertex_coord) in vertices.into_iter().enumerate() {
            let vertex_shader = &self.current_program.vertex_shader;

            let vertex_attribute_values = self
                .current_program
                .attributes
                .iter()
                .map(|attr| {
                    // todo read buffer into different data structures.
                    let v = self
                        .current_buffers
                        .get(attr.index)
                        .unwrap()
                        .get(vertex_index)
                        .unwrap();
                    ShaderData::Vec4(v.clone())
                })
                .collect();

            let projected_vertices = vertex_shader(
                &vertex_attribute_values,
                &current_uniform_values,
                vertex_coord.clone(),
            );

            current_attribute_values.push(vertex_attribute_values);

            total_vertices.push(Vertex {
                coord: Vec2{
                    value:[
                        (projected_vertices.x() / projected_vertices.w() + 1.0) / 2.0 * self.current_frame.width as f64,
                        (projected_vertices.y() / projected_vertices.w() + 1.0) / 2.0 * self.current_frame.height as f64,
                    ]
                },
                index: vertex_index,
                depth: projected_vertices.z() / projected_vertices.w(),
            })
        }

        println!("Projection complete");

        self.raster(total_vertices, current_attribute_values);
        self.fragment(&current_uniform_values);
    }

    fn clip_before_raster(&self, total_vertices: &mut Vec<Vertex>, ){
        
    }

    fn raster(
        &mut self,
        total_vertices: Vec<Vertex>,
        current_attribute_values: Vec<Vec<ShaderData>>,
    ) {
        let mut i = 0;

        let z_buffer_unit = 256.0 / (self.far - self.near);

        while i + 2 < total_vertices.len() {
            let a = total_vertices.get(i).unwrap();
            let b = total_vertices.get(i + 1).unwrap();
            let c = total_vertices.get(i + 2).unwrap();
            let ab_cross_bc = Vec2::cross(&(&b.coord - &a.coord), &(&c.coord - &b.coord));
            if ab_cross_bc < 0.0 {
                let points = raster_triangle(&a.coord, &b.coord, &c.coord);
                let attrs = [
                    current_attribute_values.get(a.index).unwrap(),
                    current_attribute_values.get(b.index).unwrap(),
                    current_attribute_values.get(c.index).unwrap(),
                ];
                for (coord, (alpha, beta, gamma)) in points {
                    let depth = a.depth * alpha + b.depth * beta + c.depth * gamma;
                    let z = ((depth - self.near) * z_buffer_unit).round() as u8;
                    let varyings = interpolate_attribute(attrs, alpha, beta, gamma);

                    if let Some(prev_pixel) = self.current_frame.get_mut(&coord) {
                        let prev_pixel_z = { prev_pixel.z };
                        if prev_pixel_z < z {
                            prev_pixel.varying = Some(varyings);
                        }
                    }
                }
            }
            println!("{} vertices remains...", total_vertices.len() - i);
            i += 3;
        }

        println!("Raster complete");
    }
    fn fragment(&mut self, current_uniform_values: &Vec<ShaderData>) {
        for mut pixel in self.current_frame.buffer.iter_mut() {
            if let Some(varyings) = &pixel.varying {
                let fragment_shader = &self.current_program.fragment_shader;
                let gl_frag_color =
                    fragment_shader(&varyings, current_uniform_values, pixel.coord.clone());
                pixel.color = (
                    (gl_frag_color.x() * 256.0).round() as u8,
                    (gl_frag_color.y() * 256.0).round() as u8,
                    (gl_frag_color.z() * 256.0).round() as u8,
                    (gl_frag_color.w() * 256.0).round() as u8,
                );
            }
        }
        println!("Fragment complete");
    }
    fn blend(&mut self) {}
}

fn interpolate_attribute<'a, 'b>(
    attrs: [&'b Vec<ShaderData>; 3],
    alpha: f64,
    beta: f64,
    gamma: f64,
) -> Vec<ShaderData> {
    let attr_len = attrs[0].len();
    let mut res = vec![];
    for i in 0..attr_len {
        let a_attr = attrs[0].get(i).unwrap() * alpha;
        let b_attr = attrs[1].get(i).unwrap() * beta;
        let c_attr = attrs[2].get(i).unwrap() * gamma;
        res.push(a_attr + b_attr + c_attr);
    }
    res
}
