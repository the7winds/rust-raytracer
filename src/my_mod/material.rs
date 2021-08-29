use std::fmt::Debug;

use rand::random;

use crate::my_mod::hittable::HitRecord;
use crate::my_mod::intensity::Intensity;
use crate::my_mod::ray::Ray;
use crate::my_mod::vec3::Vec3;
use crate::my_mod::utils::random_on_unit_sphere;
use crate::my_mod::material::Material::{Lambertian, Dielectric, Metal, Light};

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

#[derive(Debug)]
pub(crate) enum Material {
    Lambertian {
        albedo: Attenuation,
    },
    Dielectric {
        refraction_index: f32,
    },
    Metal {
        albedo: Attenuation,
        fuzz: f32,
    },
    Light {
        emit: Intensity,
    }
}

impl Material {
    pub fn lambertian(albedo: Attenuation) -> Material {
        Lambertian { albedo }
    }

    pub fn dielectric(refraction_index: f32) -> Material {
        Dielectric { refraction_index }
    }

    pub fn metal(albedo: Attenuation, fuzz: f32) -> Material {
        Metal { albedo, fuzz }
    }

    pub fn light(emit: Intensity) -> Material {
        Light { emit }
    }

    pub(crate) fn scatter(&self, input_ray: &Ray, hit_record: &HitRecord) -> ScatteringResult {
        match *self {
            Material::Lambertian { albedo } => {
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
                ScatteringResult::ScatterredRay(albedo, scattered)
            }
            Material::Dielectric { refraction_index } => {
                let refraction_index = refraction_index;
                let attenuation = Attenuation::new(1., 1., 1.);
                let angle_ratio = if hit_record.front() {
                    1. / refraction_index
                } else {
                    refraction_index
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
            Material::Metal { albedo, fuzz } => {
                let reflected = Vec3::reflect(input_ray.direction(), hit_record.normal());
                let scattered = Ray::new(
                    &hit_record.point(),
                    &(reflected + fuzz * random_on_unit_sphere()),
                );
                if Vec3::dot(reflected, hit_record.normal()) > 0. {
                    ScatteringResult::ScatterredRay(albedo, scattered)
                } else {
                    ScatteringResult::None
                }
            }
            Material::Light { emit } => {
                ScatteringResult::Light(emit)
            }
        }
    }
}
