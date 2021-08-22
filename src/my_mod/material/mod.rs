use std::fmt::Debug;

use rand::random;

use crate::my_mod::hittable::HitRecord;
use crate::my_mod::intensity::Intensity;
use crate::my_mod::ray::Ray;

pub mod dielectric;
pub mod lambertian;
pub mod light;
pub mod metal;

#[derive(Debug, Copy, Clone)]
pub struct Attenuation(f32, f32, f32);

impl Attenuation {
    pub fn new(r: f32, g: f32, b: f32) -> Attenuation {
        assert!(0. <= r && r <= 1.);
        assert!(0. <= g && g <= 1.);
        assert!(0. <= b && b <= 1.);
        Attenuation(r, g, b)
    }

    pub fn r(&self) -> f32 {
        self.0
    }

    pub fn g(&self) -> f32 {
        self.1
    }

    pub fn b(&self) -> f32 {
        self.2
    }

    pub fn random() -> Attenuation {
        Attenuation::new(random(), random(), random())
    }
}

pub enum ScatteringResult {
    None,
    Light(Intensity),
    ScatterredRay(Attenuation, Ray),
}

pub trait Material: Debug {
    fn scatter(&self, input_ray: &Ray, hit_record: &HitRecord) -> ScatteringResult;
}
