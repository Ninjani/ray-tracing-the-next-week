use rand::Rng;
use crate::rays::Ray;
use std::f32::consts::PI;
use crate::vectors::Vec3;

#[derive(Clone)]
pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
    pub lens_radius: f32,
    pub uvw: (Vec3, Vec3, Vec3),
    pub time_0: f32,
    pub time_1: f32,
}

fn random_in_unit_disk<R: Rng>(rng: &mut R) -> Vec3 {
    loop {
        let p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.) * 2. - Vec3::new(1., 1., 0.);
        if p.dot(p) < 1. {
            return p;
        }
    }
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
        time_0: f32,
        time_1: f32,
    ) -> Camera {
        let lens_radius = aperture / 2.;
        let theta = vfov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = (look_from - look_at).unit_vector();
        let u = (vup.cross(w)).unit_vector();
        let v = w.cross(u);
        let lower_left_corner =
            origin - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist;
        let horizontal = u * 2. * half_width * focus_dist;
        let vertical = v * 2. * half_height * focus_dist;
        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            lens_radius,
            uvw: (u,v,w),
            time_0,
            time_1,
        }
    }

    pub fn get_ray<R: Rng>(&self, s: f32, t: f32, rng: &mut R) -> Ray {
        let rd = random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.uvw.0 * rd.x + self.uvw.1 * rd.y;
        let time = self.time_0 + rng.gen::<f32>() * (self.time_1 - self.time_0);
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
            time,
        )
    }
}
