use std::cmp::Ordering::{Greater, Less};

use crate::my_mod::hittable::{Accuracy, HitRecord};
use crate::my_mod::ray::Ray;
use crate::my_mod::time::TimePoint;

pub trait AnimatedHittable {
    fn hit(&self, time: TimePoint, ray: &Ray, accuracy: &Accuracy) -> Option<HitRecord>;
}

pub struct AnimatedHittableList {
    pub list: Vec<Box<dyn AnimatedHittable>>
}

impl AnimatedHittable for AnimatedHittableList {
    fn hit(&self, time: TimePoint, ray: &Ray, accuracy: &Accuracy) -> Option<HitRecord> {
        self.list
            .iter()
            .flat_map(|hittable| hittable.hit(time, ray, accuracy))
            .min_by(|a, b| if a.t() < b.t() { Less } else { Greater })
    }
}