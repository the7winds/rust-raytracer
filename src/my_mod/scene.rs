use std::fmt::Debug;
use crate::my_mod::bvh::Boundable;
use crate::my_mod::hittable::Hittable;

pub trait SceneObject : Boundable + Hittable + Sync + Debug {
}

#[derive(Default)]
pub struct Scene(pub Vec<Box<dyn SceneObject>>);
//
// impl Hittable for Scene {
//     fn hit(&self, ray: &Ray, accuracy: &Accuracy) -> Option<HitRecord> {
//         self.list
//             .iter()
//             .flat_map(|hittable| hittable.hit(ray, accuracy))
//             .min_by(|a, b| if a.t < b.t { Less } else { Greater })
//     }
// }
