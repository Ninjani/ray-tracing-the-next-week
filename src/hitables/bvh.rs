use crate::bboxes::AABB;
use crate::hitables::{HitRecord, Hitable};
use crate::rays::Ray;
use rand::Rng;
use std::cmp::Ordering;
use std::rc::Rc;

macro_rules! box_compare {
    ($axis:ident, $left:ident, $right:ident) => {
        match ($left.bounding_box(0., 0.), $right.bounding_box(0., 0.)) {
            (Some(box_left), Some(box_right)) => {
                let difference = &box_left.min[$axis] - &box_right.min[$axis];
                if difference < 0. {
                    Ordering::Less
                } else if difference > 0. {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
            _ => panic!("no bounding box in bvh node constructor"),
        }
    };
}

#[derive(Clone, Debug)]
pub struct BVHNode {
    pub left: Rc<dyn Hitable>,
    pub right: Rc<dyn Hitable>,
    pub bbox: AABB,
}

impl Hitable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(&ray, t_min, t_max) {
            let hit_left = self.left.hit(&ray, t_min, t_max);
            let hit_right = self.right.hit(&ray, t_min, t_max);
            match (hit_left, hit_right) {
                (Some(left_record), Some(right_record)) => {
                    if left_record.t < right_record.t {
                        return Some(left_record);
                    } else {
                        return Some(right_record);
                    }
                }
                (Some(left_record), None) => return Some(left_record),
                (None, Some(right_record)) => return Some(right_record),
                (None, None) => return None,
            }
        }
        None
    }

    #[allow(unused_variables)]
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}

impl BVHNode {
    pub fn new(left: Rc<dyn Hitable>, right: Rc<dyn Hitable>, bbox: AABB) -> Self {
        BVHNode { left, right, bbox }
    }
    pub fn bvh_node<R: Rng>(
        list: &mut Vec<Rc<dyn Hitable>>,
        time_0: f32,
        time_1: f32,
        rng: &mut R,
    ) -> Self {
        let axis = (3. * rng.gen::<f32>()) as usize;
        list.sort_by(|left, right| box_compare!(axis, left, right));
        let n = list.len();
        let (left, right) = match n {
            1 => (Rc::clone(&list[0]), Rc::clone(&list[0])),
            2 => (Rc::clone(&list[0]), Rc::clone(&list[1])),
            _ => {
                let (l_left, l_right) = list.split_at(n / 2 as usize);
                (
                    Rc::new(BVHNode::bvh_node(&mut l_left.to_vec(), time_0, time_1, rng))
                        as Rc<dyn Hitable>,
                    Rc::new(BVHNode::bvh_node(
                        &mut l_right.to_vec(),
                        time_0,
                        time_1,
                        rng,
                    )) as Rc<dyn Hitable>,
                )
            }
        };
        match (
            left.bounding_box(time_0, time_1),
            right.bounding_box(time_0, time_1),
        ) {
            (Some(box_left), Some(box_right)) => {
                BVHNode::new(left, right, AABB::surrounding_box(&box_left, &box_right))
            }
            _ => panic!("no bounding box in bvh node constructor"),
        }
    }
}
