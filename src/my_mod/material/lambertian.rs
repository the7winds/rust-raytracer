use crate::my_mod::material::{Attenuation, Material, ScatteringResult};
use crate::my_mod::hittable::HitRecord;
use crate::my_mod::ray::Ray;
use crate::my_mod::utils::random_on_unit_sphere;

#[derive(Debug)]
pub struct Lambertian {
    albedo: Attenuation,
}

impl Lambertian {
    pub fn new(albedo: Attenuation) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> ScatteringResult {
        let scatter_direction = {
            let direction = hit_record.normal() + random_on_unit_sphere();
            let eps = 1e-5;
            if direction.length() < eps {
                hit_record.normal()
            } else {
                direction
            }
        };

        let scattered = Ray::new(&hit_record.point(), &scatter_direction);
        ScatteringResult::ScatterredRay(self.albedo, scattered)
    }
}
