use std::option::Option;

use glam::Vec3;

use crate::my_mod::material::Material;
use crate::my_mod::ray::Ray;

#[derive(Debug, Clone)]
pub struct HitRecord<'a> {
    point: Vec3,
    normal: Vec3,
    material: &'a Material,
    t: f32,
    front: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Vec3,
        normal: Vec3,
        material: &'a Material,
        t: f32,
        front: bool,
    ) -> Self {
        Self {
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

    pub fn material(&self) -> &Material {
        self.material
    }

    pub fn t(&self) -> f32 {
        self.t
    }
}

pub struct Accuracy {
    pub min: f32,
    pub max: f32,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, range: &Accuracy) -> Option<HitRecord>;
}
