mod camera;
mod engine;
mod object;
mod printer;
mod window;

use crate::printer::render_to_ppm;
use crate::printer::render_frame_to_stdout;
use camera::*;
use engine::*;
use object::*;
use window::*;

use std::fs::File;
use std::io::BufReader;

const TRANSPARENT: Vec4 = Vec4{value:[0.0,0.0,0.0,0.0]};

fn main() {

    let mut frame = Frame::new(120, 80);

    let width = frame.width;
    let height = frame.height;

    for y in 0..frame.height {
        for x in 0..frame.width {
            if let Some(mut pixel) = frame.get_mut(&(x,y)) {
                (*pixel).color.1 = (x as f64 / width as f64 * 255.99f64) as u8;
                (*pixel).color.2 = (y as f64 / height as f64 * 255.99f64) as u8;
            }
        }
    }

    let ppm = render_to_ppm(&frame);

    std::fs::write("test_ppm.ppm", ppm);

    // let file = File::open("C:\\Users\\chongchong\\Documents\\rust_projects\\soft_render\\cornell_box.obj").unwrap();
    // let mut reader1 = BufReader::new(file);
    // let (models, materials) = tobj::load_obj_buf(&mut reader1, false, |mat_path| {
    //     let mtl_file = File::open("C:\\Users\\chongchong\\Documents\\rust_projects\\soft_render\\cornell_box.mtl").unwrap();
    //     let mut reader2 = BufReader::new(mtl_file);
    //     tobj::load_mtl_buf(&mut reader2)
    // }).unwrap();

    // let mut vertices = vec!();
    // let mut colors = vec!();
    
    // for model in models.into_iter(){
    //     for i in model.mesh.indices.iter() {
    //         let coord = model.mesh.positions.get((*i as usize)..(*i as usize + 3)).unwrap();
    //         vertices.push(Vec4::new(coord[0] as f64, coord[1] as f64, coord[2] as f64, 1.0));
    //         if let Some(material_id) = model.mesh.material_id {
    //             if let Some(material) = materials.get(material_id) {
    //                 colors.push(Vec4::new(material.ambient[0] as f64, material.ambient[1] as f64, material.ambient[2] as f64, 1.0 ));
    //                 continue;
    //             }
    //         }
    //         colors.push(Vec4::new(0.0, 0.5, 0.5, 1.0));
    //     }
    // };

    // let camera_position = Vec4::new(0.0, 0.0, 500.0, 1.0);
    // let scene_up = Vec4::new(0.0, 1.0, 0.0, 1.0);
    // let mut camera_direction = &Vec4::Origin - &camera_position;
    // camera_direction.normalize();
    
    // let mut camera = Camera::new(scene_up, camera_direction, camera_position);


    // let program = Program {
    //     vertex_shader: Box::new(move |attributes, uniforms, gl_Position| {
    //         let mut projection_matrix = None;
    //         let mut model_view_matrix = None;
    //         if let Some(value) = uniforms.get(0) {
    //             if let ShaderData::Mat4(v) = value {
    //                 projection_matrix = Some(v)
    //             }
    //         }
    //         if let Some(value) = uniforms.get(1) {
    //             if let ShaderData::Mat4(v) = value {
    //                 model_view_matrix = Some(v)
    //             }
    //         }
    //         return projection_matrix.unwrap() * &(model_view_matrix.unwrap() * &gl_Position);
    //     }),
    //     fragment_shader: Box::new(move |attributes, uniforms, gl_Position| {
    //         let mut color = None;
    //         if let Some(value) = attributes.get(0) {
    //             if let ShaderData::Vec4(v) = value {
    //                 color = Some(v)
    //             }
    //         }
    //         return color.unwrap_or(&TRANSPARENT).clone();
    //     }),
    //     attributes: vec![Attribute {
    //         index: 1,
    //         name: "color".to_string(),
    //     }],
    //     uniforms: vec![
    //         Uniform {
    //             name: "projectionMatrix".to_string(),
    //             value: ShaderData::Mat4(camera.projection_matrix.clone()),
    //         },
    //         Uniform {
    //             name: "modelViewMatrix".to_string(),
    //             value: ShaderData::Mat4(camera.view_matrix.clone()),
    //         },
    //     ],
    // };
    // let mut context = Context {
    //     near: 0.1,
    //     far: 500.0,
    //     current_program: program,
    //     current_buffers: vec![],
    //     current_frame: Frame::new(1024, 768),
    // };

    // context.current_buffers.push(vertices);
    // context.current_buffers.push(colors);

    // let mut camera_velocity = Vec4::new(0.1, 0.0, 0.0, 0.0);

    // // loop {
    //     if camera.position.x() >= 1.0 || camera.position.x() <= -1.0 {
    //         camera_velocity.value[0] *= -1.0;
    //     }
    //     camera.position += &camera_velocity;
    //     camera.recompute_view_matrix();
    //     context.current_frame.clear();

    //     //update uniforms
    //     context.current_program.uniforms[1].value = ShaderData::Mat4(camera.view_matrix.clone());

    //     context.draw_triangles(0);
        
    //     print!("\x1B[{};{}H", 1, 1);

    //     // print!("\r\x1b[2J\r\x1b[H");

    //     printer::render_frame_to_stdout(&context.current_frame);

    //     std::thread::sleep(std::time::Duration::from_millis(1000 / FRAME_RATE));
    // // }
}

const FRAME_RATE:u64 = 10;