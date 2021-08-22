use crate::my_mod::{ray::Ray, vec3::Vec3};
use crate::my_mod::hittable::HitRecord;
use crate::my_mod::material::{Attenuation, Material};
use crate::my_mod::utils::random_on_unit_sphere;

use super::ScatteringResult;

#[derive(Debug)]
pub struct Metal {
    albedo: Attenuation,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Attenuation, fuzz: f32) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, input_ray: &Ray, hit_record: &HitRecord) -> ScatteringResult {
        let reflected = Vec3::reflect(input_ray.direction(), hit_record.normal());
        let scattered = Ray::new(
            &hit_record.point(),
            &(reflected + self.fuzz * random_on_unit_sphere()),
        );
        if Vec3::dot(reflected, hit_record.normal()) > 0. {
            ScatteringResult::ScatterredRay(self.albedo, scattered)
        } else {
            ScatteringResult::None
        }
    }
}
