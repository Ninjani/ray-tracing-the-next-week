use crate::bboxes::AABB;
use crate::hitables::{HitRecord, Hitable};
use crate::rays::Ray;
use crate::vectors::Vec3;
use std::f32::consts::PI;
use std::f32::MAX;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct FlipNormals {
    hitable: Rc<dyn Hitable>,
}

impl FlipNormals {
    pub fn new(hitable: Rc<dyn Hitable>) -> Self {
        FlipNormals { hitable }
    }
}

impl Hitable for FlipNormals {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self.hitable.hit(ray, t_min, t_max) {
            Some(record) => Some(HitRecord {
                normal: -record.normal,
                ..record
            }),
            None => None,
        }
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.hitable.bounding_box(t0, t1)
    }
}

#[derive(Clone, Debug)]
pub struct Translate {
    hitable: Rc<dyn Hitable>,
    offset: Vec3,
}
impl Translate {
    pub fn new(hitable: Rc<dyn Hitable>, offset: Vec3) -> Translate {
        Translate { hitable, offset }
    }
}
impl Hitable for Translate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);
        match self.hitable.hit(&moved_ray, t_min, t_max) {
            Some(record) => Some(HitRecord {
                p: record.p + self.offset,
                ..record
            }),
            None => None,
        }
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        match self.hitable.bounding_box(t0, t1) {
            Some(bbox) => Some(AABB::new(bbox.min + self.offset, bbox.max + self.offset)),
            None => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RotateY {
    hitable: Rc<dyn Hitable>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(hitable: Rc<dyn Hitable>, angle: f32) -> RotateY {
        let radians = (PI / 180.) * angle;
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut min = Vec3::new(MAX, MAX, MAX);
        let mut max = Vec3::new(-MAX, -MAX, -MAX);
        let bbox = match hitable.bounding_box(0., 1.) {
            Some(p_bbox) => {
                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let x = (i as f32) * p_bbox.max.x + (1. - (i as f32)) * p_bbox.min.x;
                            let y = (j as f32) * p_bbox.max.y + (1. - (j as f32)) * p_bbox.min.y;
                            let z = (k as f32) * p_bbox.max.z + (1. - (k as f32)) * p_bbox.min.z;
                            let new_x = cos_theta * x + sin_theta * z;
                            let new_z = -sin_theta * x + cos_theta * z;
                            let tester = Vec3::new(new_x, y, new_z);
                            for c in 0..3 {
                                if tester[c] > max[c] {
                                    max[c] = tester[c];
                                }
                                if tester[c] < min[c] {
                                    min[c] = tester[c];
                                }
                            }
                        }
                    }
                }
                Some(AABB::new(min, max))
            }
            None => None,
        };
        RotateY {
            hitable,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hitable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;
        origin.x = self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z;
        origin.z = self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z;
        direction.x = self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z;
        direction.z = self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z;
        let rotated_ray = Ray::new(origin, direction, ray.time);
        match self.hitable.hit(&rotated_ray, t_min, t_max) {
            Some(record) => {
                let mut p = record.p;
                let mut normal = record.normal;
                p.x = self.cos_theta * record.p.x + self.sin_theta * record.p.z;
                p.z = -self.sin_theta * record.p.x + self.cos_theta * record.p.z;
                normal.x = self.cos_theta * record.normal.x + self.sin_theta * record.normal.z;
                normal.z = -self.sin_theta * record.normal.x + self.cos_theta * record.normal.z;
                Some(HitRecord {
                    p,
                    normal,
                    ..record
                })
            }
            None => None,
        }
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.bbox.clone()
    }
}
