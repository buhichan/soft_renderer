mod camera;
mod engine;
mod object;
mod printer;
mod window;

mod raytrace_pipeline;

use engine::*;

fn main() {

    raytrace_pipeline::raytracing()
}