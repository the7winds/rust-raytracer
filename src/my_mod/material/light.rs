use crate::my_mod::hittable::HitRecord;
use crate::my_mod::intensity::Intensity;
use crate::my_mod::material::{Material, ScatteringResult};
use crate::my_mod::ray::Ray;

#[derive(Debug)]
pub struct Light {
    emit: Intensity,
}

impl Light {
    #[allow(dead_code)]
    pub fn new(emit: Intensity) -> Light {
        Light { emit }
    }
}

impl Material for Light {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> ScatteringResult {
        ScatteringResult::Light(self.emit)
    }
}
