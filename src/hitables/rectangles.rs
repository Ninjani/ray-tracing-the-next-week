use crate::bboxes::AABB;
use crate::hitables::{HitRecord, Hitable};
use crate::materials::Material;
use crate::rays::Ray;
use crate::vectors::Vec3;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct XYRectangle {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub material: Rc<Material>,
}
impl XYRectangle {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Rc<Material>) -> XYRectangle {
        XYRectangle {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}
impl Hitable for XYRectangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        Some(HitRecord {
            t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
            p: ray.point_at_parameter(t),
            normal: Vec3::new(0., 0., 1.),
            material: Rc::clone(&self.material),
        })
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

#[derive(Clone, Debug)]
pub struct XZRectangle {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Rc<Material>,
}
impl XZRectangle {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Rc<Material>) -> XZRectangle {
        XZRectangle {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}
impl Hitable for XZRectangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        Some(HitRecord {
            t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (z - self.z0) / (self.z1 - self.z0),
            p: ray.point_at_parameter(t),
            normal: Vec3::new(0., 1., 0.),
            material: Rc::clone(&self.material),
        })
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}

#[derive(Clone, Debug)]
pub struct YZRectangle {
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Rc<Material>,
}
impl YZRectangle {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Rc<Material>) -> YZRectangle {
        YZRectangle {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }
    }
}
impl Hitable for YZRectangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        Some(HitRecord {
            t,
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
            p: ray.point_at_parameter(t),
            normal: Vec3::new(1., 0., 0.),
            material: Rc::clone(&self.material),
        })
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
