use super::bvh::BVHNode;
use super::rectangles::*;
use super::transforms::*;
use crate::bboxes::AABB;
use crate::hitables::{HitRecord, Hitable};
use crate::materials::Material;
use crate::rays::Ray;
use crate::vectors::Vec3;
use rand::Rng;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Cuboid {
    walls: BVHNode,
    p_min: Vec3,
    p_max: Vec3,
}

impl Cuboid {
    pub fn new<R: Rng>(p_min: Vec3, p_max: Vec3, material: &Rc<Material>, rng: &mut R) -> Cuboid {
        let mut walls = Vec::with_capacity(6);
        walls.push(Rc::new(XYRectangle::new(
            p_min.x,
            p_max.x,
            p_min.y,
            p_max.y,
            p_max.z,
            Rc::clone(material),
        )) as Rc<dyn Hitable>);
        let xy_rect = XYRectangle::new(
            p_min.x,
            p_max.x,
            p_min.y,
            p_max.y,
            p_min.z,
            Rc::clone(material),
        );
        walls.push(Rc::new(FlipNormals::new(Rc::new(xy_rect))) as Rc<dyn Hitable>);
        walls.push(Rc::new(XZRectangle::new(
            p_min.x,
            p_max.x,
            p_min.z,
            p_max.z,
            p_max.y,
            Rc::clone(material),
        )) as Rc<dyn Hitable>);
        let xz_rect = XZRectangle::new(
            p_min.x,
            p_max.x,
            p_min.z,
            p_max.z,
            p_min.y,
            Rc::clone(material),
        );
        walls.push(Rc::new(FlipNormals::new(Rc::new(xz_rect))) as Rc<dyn Hitable>);
        walls.push(Rc::new(YZRectangle::new(
            p_min.y,
            p_max.y,
            p_min.z,
            p_max.z,
            p_max.x,
            Rc::clone(material),
        )) as Rc<dyn Hitable>);
        let yz_rect = YZRectangle::new(
            p_min.y,
            p_max.y,
            p_min.z,
            p_max.z,
            p_min.x,
            Rc::clone(material),
        );
        walls.push(Rc::new(FlipNormals::new(Rc::new(yz_rect))) as Rc<dyn Hitable>);
        Cuboid {
            walls: BVHNode::bvh_node(&mut walls, 0., 1., rng),
            p_min,
            p_max,
        }
    }
}

impl Hitable for Cuboid {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.walls.hit(ray, t_min, t_max)
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB::new(self.p_min, self.p_max))
    }
}
