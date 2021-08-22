use crate::my_mod::hittable::HitRecord;
use crate::my_mod::material::{Attenuation, Material, ScatteringResult};
use crate::my_mod::ray::Ray;
use crate::my_mod::vec3::Vec3;

#[derive(Debug)]
pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Dielectric {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, input_ray: &Ray, hit_record: &HitRecord) -> ScatteringResult {
        let attenuation = Attenuation::new(1., 1., 1.);
        let angle_ratio = if hit_record.front() {
            1. / self.refraction_index
        } else {
            self.refraction_index
        };
        let scattered = Ray::new(&hit_record.point(), &{
            let dir = input_ray.direction();
            let n = hit_record.normal();
            match Vec3::refract(dir, n, angle_ratio) {
                Some(direction) => direction,
                None => Vec3::reflect(dir, n),
            }
        });
        ScatteringResult::ScatterredRay(attenuation, scattered)
    }
}
