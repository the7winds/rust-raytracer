use std::cmp::Ordering::{Greater, Less};
use std::option::Option;
use std::rc::Rc;
use std::vec::Vec;

use crate::my_mod::material::Material;
use crate::my_mod::ray::Ray;
use crate::my_mod::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    material: Rc<dyn Material>,
    t: f32,
    front: bool,
}

impl HitRecord {
    pub fn new(
        point: Vec3,
        normal: Vec3,
        material: Rc<dyn Material>,
        t: f32,
        front: bool,
    ) -> HitRecord {
        HitRecord {
            point,
            normal,
            material,
            t,
            front,
        }
    }

    pub fn point(&self) -> Vec3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn front(&self) -> bool {
        self.front
    }

    pub fn material(&self) -> Rc<dyn Material> {
        self.material.clone()
    }
}

pub struct Accuracy {
    pub min: f32,
    pub max: f32,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, range: &Accuracy) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, accuracy: &Accuracy) -> Option<HitRecord> {
        self.list
            .iter()
            .flat_map(|hittable| hittable.hit(ray, accuracy))
            .min_by(|a, b| if a.t < b.t { Less } else { Greater })
    }
}
