use crate::bboxes::AABB;
use crate::hitables::{HitRecord, Hitable};
use crate::materials::Material;
use crate::rays::Ray;
use crate::textures::Texture;
use crate::vectors::Vec3;
use rand;
use std::f32::MAX;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct ConstantMedium {
    pub density: f32,
    pub boundary: Rc<dyn Hitable>,
    pub phase_function: Rc<Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Rc<dyn Hitable>, density: f32, albedo: Rc<dyn Texture>) -> Self {
        ConstantMedium {
            density,
            boundary,
            phase_function: Rc::new(Material::isotropic(albedo)),
        }
    }
}

impl Hitable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self.boundary.hit(ray, -MAX, MAX) {
            Some(record_1) => match self.boundary.hit(ray, record_1.t + 0.0001, MAX) {
                Some(record_2) => {
                    let mut record_1_t = record_1.t;
                    let mut record_2_t = record_2.t;
                    if record_1_t < t_min {
                        record_1_t = t_min;
                    }
                    if record_2_t > t_max {
                        record_2_t = t_max;
                    }
                    if record_1_t >= record_2_t {
                        return None;
                    }
                    if record_1_t < 0. {
                        record_1_t = 0.;
                    }
                    let distance_inside_boundary =
                        ray.direction.length() * (record_2_t - record_1_t);
                    let hit_distance = -(1. / self.density) * rand::random::<f32>().ln();
                    if hit_distance < distance_inside_boundary {
                        let t = record_1_t + hit_distance / ray.direction.length();
                        let p = ray.point_at_parameter(t);
                        Some(HitRecord {
                            t,
                            u: 0.,
                            v: 0.,
                            p,
                            normal: Vec3::new(1., 0., 0.),
                            material: Rc::clone(&self.phase_function),
                        })
                    } else {
                        None
                    }
                }
                None => None,
            },
            None => None,
        }
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
