use crate::vectors::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub inv_direction: Vec3,
    pub sign: [usize; 3],
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Ray {
        let inv_direction = Vec3::new(1. / direction.x, 1. / direction.y, 1. / direction.z);
        let sign = [
            (inv_direction.x < 0.) as usize,
            (inv_direction.y < 0.) as usize,
            (inv_direction.z < 0.) as usize,
        ];
        Ray {
            origin,
            direction,
            inv_direction,
            sign,
            time,
        }
    }
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
