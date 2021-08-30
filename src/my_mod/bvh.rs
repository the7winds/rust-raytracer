use std::cmp::min_by;
use std::cmp::Ordering::{Greater, Less};
use std::fmt::Debug;
use std::mem::swap;
use std::sync::Arc;

use glam::Vec3;

use crate::my_mod::bbox::BBox;
use crate::my_mod::bvh::BVH::{Leaf, Node};
use crate::my_mod::hittable::{Accuracy, HitRecord, Hittable};
use crate::my_mod::ray::Ray;
use crate::my_mod::utils::random_on_unit_sphere;
use crate::my_mod::vec3;

pub trait Boundable {
    fn bbox(&self) -> BBox;
}

pub trait BoundableAndHittable: Boundable + Hittable + Send + Sync + Debug {

}

#[derive(Debug)]
pub enum BVH {
    Node {
        child_a: Box<BVH>,
        child_b: Box<BVH>,
        bbox: BBox
    },
    Leaf {
        object: Arc<dyn BoundableAndHittable>,
        bbox: BBox
    }
}

impl BVH {
    pub fn new(objects: &[Arc<dyn BoundableAndHittable>]) -> BVH {
        if objects.is_empty() {
            panic!("There should be at least one Hittable.")
        }

        if objects.len() == 1 {
            let object = objects[0].clone();
            let bbox = object.bbox();
            Leaf {
                object,
                bbox,
            }
        } else {
            let bbox = get_bbox(&objects).unwrap();
            let (child_a, child_b) = split(objects);
            let child_a = Box::new(BVH::new(&child_a));
            let child_b = Box::new(BVH::new(&child_b));
            Node {
                child_a,
                child_b,
                bbox
            }
        }
    }
}

fn split(objects: &[Arc<dyn BoundableAndHittable>]) -> (Vec<Arc<dyn BoundableAndHittable>>, Vec<Arc<dyn BoundableAndHittable>>) {
    let len = objects.len();
    if len < 2 {
        panic!("There should be at least 2 objects.");
    }

    let mut objects = objects.to_vec();
    let axis = random_on_unit_sphere();
    objects.sort_by(|a, b| {
        if Vec3::dot(a.bbox().center(), axis) < Vec3::dot(b.bbox().center(), axis) {
            Less
        } else {
            Greater
        }
    });

    (objects[0.. len / 2].to_vec(), objects[len/2..len].to_vec())
}

impl BVH {
    fn bbox(&self) -> &BBox {
        match self {
            BVH::Node { bbox, .. } => bbox,
            BVH::Leaf { bbox, .. } => bbox
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, range: &Accuracy) -> Option<HitRecord> {
        if !has_intersection(ray, self.bbox()) {
            return None
        }

        match self {
            BVH::Node { child_a, child_b, .. } => {
                let hit_a = child_a.hit(ray, range);
                let hit_b = child_b.hit(ray, range);
                min_by(hit_a, hit_b, |a, b| {
                    fn convert(x: &Option<HitRecord>) -> f32 {
                        match x {
                            None => f32::INFINITY,
                            Some(hit) => hit.t()
                        }
                    }
                    let a = convert(a);
                    let b = convert(b);
                    if a < b {
                        Less
                    } else {
                        Greater
                    }
                })
            }
            BVH::Leaf { object, .. } => {
                object.hit(ray, range)
            }
        }
    }
}

fn has_intersection(ray: &Ray, bbox: &BBox) -> bool {
    let o = ray.origin();

    if bbox.contains(&o) {
        return true
    }

    let d = ray.direction();

    let mn = bbox.min - o;
    let mut mn: [f32; 3] = [mn.x / d.x, mn.y / d.y, mn.z / d.z];

    let mx = bbox.max - o;
    let mut mx: [f32; 3] = [mx.x / d.x, mx.y / d.y, mx.z / d.z];

    if d.x < 0. {
        swap(&mut mn[0], &mut mx[0]);
    }
    if d.y < 0. {
        swap(&mut mn[1], &mut mx[1]);
    }
    if d.z < 0. {
        swap(&mut mn[2], &mut mx[2]);
    }

    let mn = mn.iter().max_by(|a, b| if a < b { Less } else { Greater }).unwrap();
    let mx = mx.iter().min_by(|a, b| if a <= b { Less } else { Greater }).unwrap();

    0. <= *mn && mn <= mx
}

fn get_bbox(objects: &[Arc<dyn BoundableAndHittable>]) -> Option<BBox> {
    Some(objects.iter()
        .map(|h| h.bbox())
        .fold(BBox { min: vec3::zero(), max: vec3::zero() }, |acc, bbox| BBox::merge(&acc, &bbox)))
}
