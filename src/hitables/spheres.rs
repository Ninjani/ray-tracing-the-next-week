use crate::bboxes::AABB;
use crate::hitables::{HitRecord, Hitable};
use crate::materials::Material;
use crate::rays::Ray;
use crate::vectors::Vec3;
use std::f32::consts::PI;
use std::rc::Rc;

fn get_sphere_uv(p: &Vec3) -> (f32, f32) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    (1. - (phi + PI) / (2. * PI), (theta + PI / 2.) / PI)
}

#[derive(Clone, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Rc<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let (a, b, c) = (ray.direction.dot(ray.direction), oc.dot(ray.direction), oc.dot(oc) - self.radius * self.radius);
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let temp = (-b - discriminant.sqrt()) / a;
            let p = ray.point_at_parameter(temp);
            let (u, v) = get_sphere_uv(&((p - self.center) / self.radius));
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    u,
                    v,
                    p,
                    normal: (ray.point_at_parameter(temp) - self.center) / self.radius,
                    material: Rc::clone(&self.material),
                });
            }
        }
        None
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}

#[derive(Clone, Debug)]
pub struct MovingSphere {
    pub center_0: Vec3,
    pub center_1: Vec3,
    pub time_0: f32,
    pub time_1: f32,
    pub radius: f32,
    pub material: Rc<Material>,
}

impl MovingSphere {
    pub fn new(
        center_0: Vec3,
        center_1: Vec3,
        time_0: f32,
        time_1: f32,
        radius: f32,
        material: Rc<Material>,
    ) -> MovingSphere {
        MovingSphere {
            center_0,
            center_1,
            time_0,
            time_1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f32) -> Vec3 {
        self.center_0
            + (self.center_1 - self.center_0) * ((time - self.time_0) / (self.time_1 - self.time_0))
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let (a, b, c) = (ray.direction.dot(ray.direction), oc.dot(ray.direction), oc.dot(oc) - self.radius * self.radius);
        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let temp = (-b - discriminant.sqrt()) / a;
            let p = ray.point_at_parameter(temp);
            let (u, v) = get_sphere_uv(&((p - self.center(ray.time)) / self.radius));
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    u,
                    v,
                    p,
                    normal: (ray.point_at_parameter(temp) - self.center(ray.time)) / self.radius,
                    material: Rc::clone(&self.material),
                });
            }
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB::surrounding_box(
            &AABB::new(
                self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
                self.center(t0) + Vec3::new(self.radius, self.radius, self.radius),
            ),
            &AABB::new(
                self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
                self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
            ),
        ))
    }
}
