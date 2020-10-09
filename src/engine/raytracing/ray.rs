use super::Material;
use crate::Scene;
use crate::Sphere;
use crate::Vec4;
use crate::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec4,
    pub dir: Vec4,
}

#[derive(Debug)]
pub struct IntersectionResult<'a> {
    pub sphere: &'a Sphere,
    pub point: Vec4,
    pub normal: Vec4,
    pub refraction_ratio: f64,
}

impl Ray {
    const NEAR_DISTANCE: f64 = 0.001;
    const FAR_DISTANCE: f64 = f64::INFINITY;
    pub fn intersect<'a>(&self, scene: &'a Scene) -> Option<IntersectionResult<'a>> {
        /*
         * 问题: 没有排序
         */

        let mut res: Option<IntersectionResult> = None;
        let mut cur_nearest = Ray::FAR_DISTANCE;

        for sphere in scene.objects.iter() {
            let ro = sphere.origin - self.origin;

            if ro.length() - sphere.radius < -Ray::NEAR_DISTANCE {
                //射线在球里面
                /*
                 * inside
                 *
                 *
                 *    r----d------p  this is the ray --->
                 *         |     / \
                 *         |    /   \
                 *         |   /     reflection
                 *         |  /
                 *         | /
                 *         |/
                 *         o this is the sphere origin
                 */
                let dist_rd = ro * self.dir;
                let rd = self.dir * dist_rd;
                let od = rd - ro;
                let dist_od = (od * od).sqrt();

                let sine_inc = dist_od / sphere.radius;
                let dist_pd = sphere.radius * sine_inc.acos().sin();
                let dp = self.dir * dist_pd;
                let op = od + dp;
                let p = sphere.origin + op;
                let normal = (op * -1.0).normalize();
                let refraction_ratio = sphere.material.refraction;

                res = Some(IntersectionResult {
                    sphere,
                    point: p,
                    normal,
                    refraction_ratio,
                });
                break;
            }

            /*
             * 在球外面
             *        l
             *       / <- this is reflection
             *     /
             * r-p------d-------  this is the ray --->
             *    \     |
             *     \    |
             *      \   |
             *       \  |
             *        \ |
             *         \|
             *          o this is the sphere origin
             */
            let dist_rd = ro * self.dir;
            if dist_rd > Ray::NEAR_DISTANCE {
                let rd = self.dir * dist_rd;
                let od = rd - ro;
                let dist_od = (od * od).sqrt();
                let sine_inc = dist_od / sphere.radius;
                let dist_pd = sphere.radius * sine_inc.acos().sin();
                let dist_rp = dist_rd - dist_pd;
                if dist_rp < cur_nearest {
                    let dp = self.dir * dist_pd * -1.0;
                    let op = od + dp;
                    let p = sphere.origin + op;
                    let normal = op.normalize();

                    let refraction_ratio = 1.0 / sphere.material.refraction;

                    res = Some(IntersectionResult {
                        sphere,
                        point: p,
                        normal,
                        refraction_ratio,
                    });
                    cur_nearest = dist_rp;
                }
            }
        }
        res
    }
}

#[test]
fn test_intersect_ray() {
    let epsilon = 1e-7;
    let mut scene = Scene { objects: vec![] };
    scene.add_sphere(Vec4::ORIGIN, 11.0, Material::GLASS, Vec3::ORIGIN);

    {
        let ray1 = Ray {
            origin: Vec4::new(10.0, 10.0, 10.0, 1.0),
            dir: Vec4::new(0.0, -10.0, -10.0, 1.0).normalize(),
        };
        let res = ray1.intersect(&scene);
        assert!(res.is_some());
        if let Some(intersect) = res {
            let IntersectionResult { point, normal, .. } = intersect;
            println!("{:?}", intersect);
            assert!((point.length() - 11.0).abs() < epsilon);
            assert!((normal.x() - 10.0 / 11.0).abs() < epsilon);
        }
    }

    {
        let ray1 = Ray {
            origin: Vec4::new(10.0, 10.0, 10.0, 1.0),
            dir: Vec4::new(0.0, 1.0, 1.0, 1.0).normalize(),
        };
        let res = ray1.intersect(&scene);
        assert!(res.is_none());
    }

    {
        let ray1 = Ray {
            origin: Vec4::new(5.0, 0.0, 0.0, 1.0),
            dir: Vec4::new(0.0, 1.0, 0.0, 1.0).normalize(),
        };
        let res = ray1.intersect(&scene);
        println!("{:?}", res);
        assert!(res.is_some());
        assert_eq!(res.unwrap().point.x(), 5.0);
    }
}
