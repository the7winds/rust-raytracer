use std::cmp::min_by;
use std::cmp::Ordering::{Greater, Less};
use std::fmt::Debug;
use std::mem::swap;

use glam::Vec3;

use crate::my_mod::bbox::BBox;
use crate::my_mod::bvh::BVH::{Leaf, Node};
use crate::my_mod::hittable::{Accuracy, HitRecord, Hittable};
use crate::my_mod::ray::Ray;
use crate::my_mod::scene::SceneObject;
use crate::my_mod::utils::random_on_unit_sphere;
use crate::my_mod::vec3;
use crate::Scene;

pub trait Boundable {
    fn bbox(&self) -> BBox;
}


#[derive(Debug)]
enum BVH {
    Node {
        child_a: Box<BVH>,
        child_b: Box<BVH>,
        bbox: BBox
    },
    Leaf {
        index: usize,
        bbox: BBox
    }
}

impl BVH {
    fn new_impl(objects: &[Box<dyn SceneObject>], indexes: Vec<usize>) -> Self {
        if indexes.is_empty() {
            panic!("There should be at least one Hittable.")
        }

        if indexes.len() == 1 {
            let index = indexes[0];
            let bbox = objects[index].bbox();
            Leaf {
                index,
                bbox,
            }
        } else {
            let bbox = indexes.iter()
                .map(|i| objects[*i].bbox())
                .fold(BBox { min: vec3::zero(), max: vec3::zero() }, |acc, bbox| BBox::merge(&acc, &bbox));
            let (child_a, child_b) = split(objects, indexes);
            let child_a = Box::new(BVH::new_impl(objects, child_a));
            let child_b = Box::new(BVH::new_impl(objects, child_b));
            Node {
                child_a,
                child_b,
                bbox
            }
        }
    }

    fn new(scene: &Scene) -> Self {
        let Scene(objects) = scene;
        let indexes = (0..objects.len()).collect::<Vec<usize>>();
        Self::new_impl(objects, indexes)
    }
}

fn split(objects: &[Box<dyn SceneObject>], mut indexes: Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    let len = indexes.len();
    if len < 2 {
        panic!("There should be at least 2 objects.");
    }

    let axis = random_on_unit_sphere();
    indexes.sort_by(|a, b| {
        let a = objects.get(*a).unwrap();
        let b = objects.get(*b).unwrap();
        if Vec3::dot(a.bbox().center(), axis) < Vec3::dot(b.bbox().center(), axis) {
            Less
        } else {
            Greater
        }
    });

    (indexes[0.. len / 2].to_vec(), indexes[len/2..len].to_vec())
}

impl BVH {
    fn bbox(&self) -> &BBox {
        match self {
            BVH::Node { bbox, .. } => bbox,
            BVH::Leaf { bbox, .. } => bbox
        }
    }
}

pub struct BVHScene<'a> {
    bvh: BVH,
    scene: &'a Scene
}

impl BVHScene<'_> {
    pub fn new(scene: &Scene) -> BVHScene {
        BVHScene {
            bvh: BVH::new(scene),
            scene
        }
    }
}

fn find_hit<'a>(node: &BVH, scene: &'a Scene, ray: &Ray, range: &Accuracy) -> Option<HitRecord<'a>> {
    if !has_intersection(ray, node.bbox()) {
        return None
    }

    match node {
        BVH::Node { child_a, child_b, .. } => {
            let hit_a =  find_hit(child_a, scene, ray, range);
            let hit_b =  find_hit(child_b, scene, ray, range);
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
        BVH::Leaf { index, .. } => {
            let Scene(scene) = scene;
            scene[*index].hit(ray, range)
        }
    }
}


impl Hittable for BVHScene<'_> {
    fn hit(&self, ray: &Ray, range: &Accuracy) -> Option<HitRecord> {
        find_hit(&self.bvh, self.scene, ray, range)
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
