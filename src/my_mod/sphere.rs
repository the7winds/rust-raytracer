use crate::my_mod::{ray::Ray, vec3::Vec3};
use crate::my_mod::hittable::{Accuracy, HitRecord, Hittable};
use crate::my_mod::material::Material;

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center: *center,
            radius,
            material,
        }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, accuracy: &Accuracy) -> Option<HitRecord> {
        let oc = ray.origin() - self.center();
        let a = Vec3::dot(ray.direction(), ray.direction());
        let b = 2. * Vec3::dot(oc, ray.direction());
        let c = Vec3::dot(oc, oc) - self.radius().powi(2);
        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            return None;
        }

        let discriminant = discriminant.sqrt();

        let Accuracy { min, max } = *accuracy;

        let t = {
            let root = (-b - discriminant) / (2. * a);
            if root < min || max < root {
                let root = (-b + discriminant) / (2. * a);
                if root < min || max < root {
                    return None;
                } else {
                    root
                }
            } else {
                root
            }
        };

        let point = ray.at(t);
        let outward_normal = (ray.at(t) - self.center()).normalize();
        let front = Vec3::dot(ray.direction(), outward_normal) < 0.;
        let normal = if front {
            outward_normal
        } else {
            -outward_normal
        };

        let hit_record = HitRecord::new(point, normal, &self.material, t, front);

        Some(hit_record)
    }
}
