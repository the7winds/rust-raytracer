use std::rc::Rc;

use crate::my_mod::animated_hittable::AnimatedHittable;
use crate::my_mod::hittable::{Accuracy, HitRecord, Hittable};
use crate::my_mod::material::Material;
use crate::my_mod::ray::Ray;
use crate::my_mod::sphere::Sphere;
use crate::my_mod::time::TimePoint;
use crate::my_mod::vec3::Vec3;

pub struct AnimatedSphere {
    center: Vec3,
    radius: f32,
    material: Material,
    translation: Box<dyn Fn(TimePoint) -> Vec3>
}

impl AnimatedSphere {
    pub fn new(center: Vec3, radius: f32, material: Material, translation: impl 'static + Fn(TimePoint) -> Vec3) -> AnimatedSphere {
        AnimatedSphere {
            center,
            radius,
            material,
            translation: Box::new(translation)
        }
    }
}

impl AnimatedHittable for AnimatedSphere {
    fn hit(&self, time: TimePoint, ray: &Ray, accuracy: &Accuracy) -> Option<HitRecord> {
        let translation = (self.translation)(time);
        let center = self.center + translation;
        Sphere::new(
            &center,
            self.radius,
            self.material.clone()
        ).hit(ray, accuracy)
    }
}