pub mod bvh;
pub mod cuboids;
pub mod mediums;
pub mod rectangles;
pub mod spheres;
pub mod transforms;

use crate::bboxes::AABB;
use crate::materials::Material;
use crate::rays::Ray;
use crate::vectors::Vec3;
use std::f32::MAX;
use rand::Rng;
use std::rc::Rc;
use std::fmt::Debug;

const MAX_DEPTH: i32 = 60;

pub fn color_world<R: Rng, H: Hitable>(ray: &Ray, world: &H, depth: i32, external_light: bool, rng: &mut R) -> Vec3 {
    match world.hit(ray, 0.001, MAX) {
        Some(record) => {
            let emitted = record.material.emitted(record.u, record.v, record.p);
            match record.material.scatter(ray, &record, rng) {
                Some((attenuation, scattered)) => {
                    if depth < MAX_DEPTH {
                        emitted + color_world(&scattered, world, depth + 1, external_light, rng) * attenuation
                    } else {
                        emitted
                    }
                }
                None => emitted,
            }
        }
        None => {
            if external_light {
                let unit_direction = ray.direction.unit_vector();
                let t = 0.5 * (unit_direction.y + 1.);
                Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t
            } else {
                Vec3::empty()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Rc<Material>,
}

pub trait Hitable: Debug {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

#[derive(Clone, Debug)]
pub struct HitableList {
    pub objects: Vec<Rc<dyn Hitable>>,
}

impl HitableList {
    pub fn new(objects: Vec<Rc<dyn Hitable>>) -> HitableList {
        HitableList { objects }
    }

    pub fn empty() -> HitableList {
        HitableList {
            objects: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> HitableList {
        HitableList {
            objects: Vec::with_capacity(capacity),
        }
    }

    pub fn push<H: 'static + Hitable>(&mut self, object: H) {
        self.objects.push(Rc::new(object));
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut temp_record: Option<HitRecord> = None;
        for object in &self.objects {
            if let Some(record) = object.hit(ray, t_min, closest_so_far) {
                    closest_so_far = record.t;
                    temp_record = Some(record);
            }
        }
        temp_record
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.objects.is_empty() {
            return None
        }
        match self.objects[0].bounding_box(t0, t1) {
            Some(bbox) => {
                let mut surrounding_box = bbox.clone();
                for i in 1..self.objects.len() {
                    match self.objects[i].bounding_box(t0, t1) {
                        Some(bbox_i) => surrounding_box = AABB::surrounding_box(&surrounding_box, &bbox_i),
                        None => return None
                    }
                }
                Some(surrounding_box)
            },
            None => None,
        }
    }
}