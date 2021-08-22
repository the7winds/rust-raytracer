use std::f32::consts::PI;

pub enum Angle {
    Degrees(f32),
    #[allow(dead_code)]
    Radians(f32),
}

impl Angle {
    pub fn radians(&self) -> f32 {
        match *self {
            Angle::Degrees(val) => PI * val / 180.,
            Angle::Radians(val) => val,
        }
    }

    #[allow(dead_code)]
    pub fn degrees(&self) -> f32 {
        match *self {
            Angle::Degrees(val) => val,
            Angle::Radians(val) => PI * val / 180.,
        }
    }
}
