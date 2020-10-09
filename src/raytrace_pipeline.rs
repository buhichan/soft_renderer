use crate::camera::Camera;
use crate::object::Object;
use crate::printer::render_to_ppm;
use crate::Frame;
use crate::IntersectionResult;
use crate::Material;
use crate::Ray;
use crate::Scene;
use crate::Vec3;
use crate::Vec4;
use rand::Rng;

pub fn raytracing() {
    let width_pixel = 1024;
    let height_pixel = 768;

    let mut frame = Frame::new(width_pixel, height_pixel);

    let camera_position = Vec4::new(0.0, 300.0, 500.0, 1.0);
    let camera_right = Vec4::new(1.0, 0.0, 0.0, 1.0).normalize();
    let camera_direction = (Vec4::ORIGIN - camera_position).normalize();
    let camera_up = Vec4::cross(camera_right, camera_direction);
    let mut camera = Camera::new(camera_up, camera_direction, camera_position);
    camera.aspect_ratio = width_pixel as f64 / height_pixel as f64;

    let view_plane_left_unit = camera_right * (camera.fov / 2.0).cos();
    let view_plane_up_unit = camera_up * (camera.fov / 2.0).cos() / camera.aspect_ratio * -1.0;

    println!("{:?} {:?}", view_plane_left_unit, view_plane_up_unit);

    let width = frame.width as f64;
    let height = frame.height as f64;

    const SUPERSAMPLING: u8 = 10;

    let mut scene = Scene { objects: vec![] };

    scene.add_sphere(Vec4::new(80.0, 100.0, -200.0, 1.0), 100.0, Material::RUBBER, Vec3::new(0.0, 1.0, 0.0));
    scene.add_sphere(Vec4::new(0.0, 100.0, 50.0, 1.0), 100.0, Material::GLASS, Vec3::WHITE);
    scene.add_sphere(Vec4::new(-150.0, 200.0, -500.0, 1.0), 200.0, Material::MIRROR, Vec3::WHITE);
    scene.add_sphere(Vec4::new(100.0, 50.0, 200.0, 1.0), 50.0, Material::RUBBER, Vec3::new(1.0, 0.0, 0.0));
    scene.add_sphere(Vec4::new(350.0, 200.0, -300.0, 1.0), 200.0, Material::METAL, Vec3::new(0.5, 0.5, 1.0));
    scene.add_sphere(Vec4::new(-120.0, 200.0, 100.0, 1.0), 40.0, Material::WATER, Vec3::new(1.0, 1.0, 1.0));
    scene.add_sphere(Vec4::new(0.0, 300.0, 0.0, 1.0), 50.0, Material::RUBBER, Vec3::new(1.0, 1.0, 0.0));
    scene.add_sphere(
        Vec4::new(0.0, -800000.0, 0.0, 1.0),
        800000.0,
        Material::RUBBER,
        Vec3::WHITE,
    );

    for y in 0..frame.height {
        for x in 0..frame.width {
            let mut sample_points = vec![];

            for x_offset in 0..SUPERSAMPLING {
                for y_offset in 0..SUPERSAMPLING {
                    sample_points.push((
                        x as f64 + x_offset as f64 / SUPERSAMPLING as f64,
                        y as f64 + y_offset as f64 / SUPERSAMPLING as f64,
                    ))
                }
            }

            let mut rays = sample_points
                .into_iter()
                .map(|(x, y)| {
                    let x_f = x / width * 2.0 - 1.0;
                    let y_f = y / height * 2.0 - 1.0;
                    let dir =
                        (camera_direction + view_plane_left_unit * x_f + view_plane_up_unit * y_f)
                            .normalize();
                    Ray {
                        origin: camera_position.clone(),
                        dir: dir,
                    }
                })
                .collect::<Vec<Ray>>();

            let mut sample_colors: Vec<Vec3> = vec![];
            loop {
                if rays.len() == 0 {
                    break;
                }
                let rays_iter = rays.into_iter();
                rays = vec![];
                for ray in rays_iter {
                    sample_colors.push(get_color(ray, &scene, 1.0, false))
                }
            }
            if let Some(mut pixel) = frame.get_mut(&(x, y)) {
                let sample_size = sample_colors.len() as f64;
                let frag_color = sample_colors
                    .into_iter()
                    .fold(Vec3::ORIGIN, |res, color| res + color / sample_size);
                //sqrt is gamme correction
                (*pixel).color = (
                    (frag_color.x().sqrt() * 255.99) as u8,
                    (frag_color.y().sqrt() * 255.99) as u8,
                    (frag_color.z().sqrt() * 255.99) as u8,
                    255,
                );
            }
            // if let Some(mut pixel) = frame.get_mut(&(x,y)) {
            //     (*pixel).color.1 = (x as f64 / width as f64 * 255.99f64) as u8;
            //     (*pixel).color.2 = (y as f64 / height as f64 * 255.99f64) as u8;
            // }
        }
        print!("\rPROGRESS: {:0>3}/{}", y + 1, height);
    }

    let ppm = render_to_ppm(&frame);

    std::fs::write("./output/test_ppm.ppm", ppm).unwrap();
}

fn get_color(ray: Ray, scene: &Scene, intensity: f64, simple_mode: bool) -> Vec3 {
    if intensity < 0.005 {
        return Vec3::BLACK;
    }
    if let Some(intersection) = ray.intersect(scene) {
        let IntersectionResult {
            sphere,
            point,
            normal,
            refraction_ratio,
            ..
        } = intersection;
        let Material {
            reflectance,
            refraction,
            reflect_fuzziness,
            diffuse,
            ..
        } = sphere.material;
        //这个0.5 表示我们的材料吸收一半光照
        let mut cur_color = Vec3::ORIGIN;
        if diffuse > 0.0 {
            let num_of_diffuse_rays;
            if simple_mode {
                num_of_diffuse_rays = 1
            } else {
                num_of_diffuse_rays = 3
            }
            for _i in 0..num_of_diffuse_rays {
                let epsilon = 0.002;
                let diffuse_dir = (normal + noise_3d(normal.xyz(), 0.6)).normalize();
                let diffuse_ray = Ray {
                    origin: point + diffuse_dir * epsilon,
                    dir: diffuse_dir,
                };
                cur_color = cur_color + get_color(diffuse_ray, scene, diffuse * intensity, true) / num_of_diffuse_rays as f64;
            }
        }
        if reflectance > 0.0 {
            let epsilon = 0.002;
            let mut refl_dir = (ray.dir - normal * (ray.dir * normal) * 2.0).normalize();
            refl_dir += noise_3d(refl_dir.xyz(), reflect_fuzziness);
            if refl_dir * normal < 0.0 {
                //如果小于0 表示反射光线被反射到法线的相反方向了.
                refl_dir = refl_dir - (refl_dir - normal) * 0.5;
            }
            let reflect_ray = Ray {
                origin: point + refl_dir * epsilon,
                dir: refl_dir.normalize(),
            };
            cur_color = cur_color + get_color(reflect_ray, scene, reflectance * intensity, true);
        }
        if refraction > 0.0 {
            let epsilon = 0.002;
            let dt = ray.dir * normal;
            let discreminant = 1.0 - refraction_ratio * refraction_ratio * (1.0 - dt * dt);
            if discreminant > 0.0 {
                let refr_dir =
                    (ray.dir - normal * dt) * refraction_ratio - normal * discreminant.sqrt();
                let refract_ray = Ray {
                    origin: point + refr_dir * epsilon,
                    dir: refr_dir,
                };
                // 折射系数不能超过0.8
                let refraction_factor = f64::min(1.0 - reflectance - diffuse, 0.8);
                // println!(">{:?}\n>>{:?}\n>>>{:?}", refract_ray, ray, sphere);
                cur_color = cur_color
                    + get_color(
                        refract_ray,
                        scene,
                        refraction_factor * intensity,
                        true,
                    );
            }
        }
        return Vec3::new(
            sphere.color.value[0] * cur_color.value[0],
            sphere.color.value[1] * cur_color.value[1],
            sphere.color.value[2] * cur_color.value[2],
        ) * intensity;
        // return cur_color * intensity;
    } else {
        // 天空颜色
        let sky_color_bottom: Vec3 = Vec3::new(0.9, 0.9, 0.9);
        let sky_color_top: Vec3 = Vec3::new(0.5, 0.7, 0.9);
        let t = 0.5 * (ray.dir.y() + 1.0);
        return sky_color_bottom * (1.0 - t) + sky_color_top * t;
    }
}

fn noise_3d(mut normal: Vec3, radius: f64) -> Vec4 {
    let mut point;
    loop {
        // normal = normal * 0.5 + 0.5;
        // let v1 = perlin_noise(8.3 + normal.x(), 12.3 + normal.y(), 10.3 + normal.z(), 0.001212);
        // let v2 = perlin_noise(12.4 + normal.x(), 43.0 + normal.y(), 12.3 + normal.z(), 0.002482);
        // let v3 = perlin_noise(32.3 + normal.x(), 9.4 + normal.y(), 0.53 + normal.z(), 0.00225);
        // point = Vec4::new(v1 * 2.0 - 1.0, v2 * 2.0 - 1.0, v3 * 2.0 - 1.0, 1.0);
        point = Vec4::new(
            rand::random::<f64>() * 2.0 - 1.0,
            rand::random::<f64>() * 2.0 - 1.0,
            rand::random::<f64>() * 2.0 - 1.0,
            1.0,
        );
        if point.length() <= 1.0 {
            point = point * radius;
            break;
        }
    }
    point
}
