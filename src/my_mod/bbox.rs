use glam::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct BBox {
    pub min: Vec3,
    pub max: Vec3
}

impl BBox {
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) / 2.
    }

    pub fn contains(&self, z: &Vec3) -> bool {
        self.min.x <= z.x && z.x <= self.max.x
            && self.min.y <= z.y && z.y <= self.max.y
            && self.min.z <= z.z && z.z <= self.max.z
    }

    pub fn merge(a: &BBox, b: &BBox) -> BBox {
        BBox {
            min: Vec3::new(
                 f32::min(a.min.x, b.min.x),
                 f32::min(a.min.y, b.min.y),
                 f32::min(a.min.z, b.min.z)
            ),
            max: Vec3::new(
                f32::max(a.max.x, b.max.x),
                f32::max(a.max.y, b.max.y),
                f32::max(a.max.z, b.max.z)
            )
        }
    }
}