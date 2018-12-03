use crate::rays::Ray;
use std::mem::swap;
use std::ops::Index;
use crate::vectors::Vec3;

#[derive(Clone, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl Index<usize> for AABB {
    type Output = Vec3;
    fn index(&self, i: usize) -> &Vec3 {
        match i {
            0 => &self.min,
            1 => &self.max,
            _ => panic!("Index out of range"),
        }
    }
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB { min, max }
    }
    pub fn old_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1. / &ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;
            if inv_d < 0. {
                swap(&mut t0, &mut t1);
            }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut t0 = (self[ray.sign[0]].x - ray.origin.x) * ray.inv_direction.x;
        let mut t1 = (self[1 - ray.sign[0]].x - ray.origin.x) * ray.inv_direction.x;
        let ty0 = (self[ray.sign[1]].y - ray.origin.y) * ray.inv_direction.y;
        let ty1 = (self[1 - ray.sign[1]].y - ray.origin.y) * ray.inv_direction.y;
        if t0 > ty1 || ty0 > t1 {
            return false;
        }
        if ty0 > t0 {
            t0 = ty0;
        }
        if ty1 < t1 {
            t1 = ty1;
        }
        let tz0 = (self[ray.sign[2]].z - ray.origin.z) * ray.inv_direction.z;
        let tz1 = (self[1 - ray.sign[2]].z - ray.origin.z) * ray.inv_direction.z;
        if t0 > tz1 || tz0 > t1 {
            return false;
        }
        if tz0 > t0 {
            t0 = tz0;
        }
        if tz1 < t1 {
            t1 = tz1;
        }
        t0 < t_max && t1 > t_min
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let small = Vec3::new(
            box0.min.x.min(box1.min.x),
            box0.min.y.min(box1.min.y),
            box0.min.z.min(box1.min.z),
        );
        let big = Vec3::new(
            box0.max.x.max(box1.max.x),
            box0.max.y.max(box1.max.y),
            box0.max.z.max(box1.max.z),
        );
        AABB::new(small, big)
    }
}
