use crate::my_mod::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: &Vec3, dir: &Vec3) -> Ray {
        Ray {
            orig: *orig,
            dir: dir.normalize(),
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.orig + t * self.dir
    }
}
